# Chess Programming Concepts

Reference guide for chess programming techniques relevant to ChessWAV.

---

## Board Representation

The board representation is the most fundamental data structure in any chess program. There are two families of approaches: **square-centric** (what's on each square?) and **piece-centric** (where is each piece?). Most engines use a hybrid of both.

### Mailbox (Square-Centric)

The simplest approach: an array where each element represents a square.

**8x8 Mailbox**

```
Index: rank * 8 + file
Array: [Option<(Piece, Color)>; 64]
```

A flat array of 64 elements. Each entry stores the piece occupying that square (or empty). Advantage: O(1) lookup by square. Disadvantage: finding all pieces of a type requires scanning all 64 squares.

A 2D variant `[[Piece; 8]; 8]` indexes by `[rank][file]` and avoids the multiply but is otherwise equivalent.

**10x12 Mailbox**

```
  -1 -1 -1 -1 -1 -1 -1 -1 -1 -1
  -1 -1 -1 -1 -1 -1 -1 -1 -1 -1
  -1  R  N  B  Q  K  B  N  R -1
  -1  P  P  P  P  P  P  P  P -1
  -1  .  .  .  .  .  .  .  . -1
  -1  .  .  .  .  .  .  .  . -1
  -1  .  .  .  .  .  .  . . -1
  -1  .  .  .  .  .  .  .  . -1
  -1  p  p  p  p  p  p  p  p -1
  -1  r  n  b  q  k  b  n  r -1
  -1 -1 -1 -1 -1 -1 -1 -1 -1 -1
  -1 -1 -1 -1 -1 -1 -1 -1 -1 -1
```

A 120-element array (10 columns x 12 rows) that pads the 8x8 board with sentinel values (-1). When generating moves, if a piece lands on a sentinel, the move is off-board. This avoids explicit boundary checks — just test if `board[destination] == SENTINEL`.

**0x88**

Uses a 128-element array (16 columns x 8 rows). Only half the array represents valid squares. The key trick: a square index is valid if and only if `(index & 0x88) == 0`.

```
Square index = 16 * rank + file

Valid:   0x00..0x07, 0x10..0x17, ..., 0x70..0x77
Invalid: 0x08..0x0F, 0x18..0x1F, ...  (bit 3 set = off-board right)
         0x80+                         (bit 7 set = off-board top)
```

Advantages:
- Off-board detection with a single bitwise AND: `if (sq & 0x88) != 0 { invalid }`
- Square difference uniquely encodes direction and distance (useful for attack tables)
- No sentinel memory waste — invalid slots are simply unused

The difference between two 0x88 squares maps to a unique value in range 0..238, enabling a 256-entry lookup table for attacks instead of the 4096-entry table needed with raw coordinate differences.

### Bitboards (Piece-Centric)

A bitboard is a single `u64` where each bit represents one square of the board. Bit 0 = a1, bit 1 = b1, ..., bit 63 = h8.

```
  8 | 56 57 58 59 60 61 62 63
  7 | 48 49 50 51 52 53 54 55
  6 | 40 41 42 43 44 45 46 47
  5 | 32 33 34 35 36 37 38 39
  4 | 24 25 26 27 28 29 30 31
  3 | 16 17 18 19 20 21 22 23
  2 |  8  9 10 11 12 13 14 15
  1 |  0  1  2  3  4  5  6  7
    +---------------------------
       a  b  c  d  e  f  g  h
```

A full position requires **12 bitboards** — one for each (piece-type, color) combination:

```rust
struct Bitboards {
    pieces: [[u64; 6]; 2],  // [Color][PieceType]
    // pieces[WHITE][PAWN]   = 0x000000000000FF00  (initial white pawns)
    // pieces[BLACK][KNIGHT] = 0x4200000000000000  (initial black knights)
}
```

Plus **aggregate occupancy** boards for fast queries:

```rust
white_occupancy: u64,  // OR of all white piece bitboards
black_occupancy: u64,  // OR of all black piece bitboards
all_occupancy: u64,    // white_occupancy | black_occupancy
```

**Core operations** use bitwise instructions:

| Operation | Code | Purpose |
|-----------|------|---------|
| Set a bit | `bb \|= 1u64 << sq` | Place piece on square |
| Clear a bit | `bb &= !(1u64 << sq)` | Remove piece from square |
| Test a bit | `bb & (1u64 << sq) != 0` | Is square occupied? |
| Intersection | `bb1 & bb2` | Squares common to both |
| Union | `bb1 \| bb2` | Squares in either |
| Complement | `!bb` | Empty squares |
| Pop count | `bb.count_ones()` | Number of pieces |
| Bitscan | `bb.trailing_zeros()` | Index of first piece |
| Isolate lowest | `bb & bb.wrapping_neg()` | Single-piece bitboard |
| Clear lowest | `bb & (bb - 1)` | Remove first piece |

**Iterating over pieces** in a bitboard:

```rust
let mut pieces = bitboard;
while pieces != 0 {
    let square = pieces.trailing_zeros() as u8;
    // process square
    pieces &= pieces - 1; // clear lowest set bit
}
```

**Advantages**:
- Parallel operations on all 64 squares simultaneously
- Attack computation via shifts and masks
- Set operations (intersection, union) in a single CPU instruction
- Natural in Rust: `u64` is a native type with built-in bit manipulation methods

**Disadvantages**:
- "What piece is on square X?" requires checking up to 12 bitboards (unless paired with a mailbox)
- Sliding piece attacks need lookup tables or iterative computation
- More complex to debug (raw u64 values aren't human-readable)

### Piece Lists

An explicit list of active pieces per color:

```rust
struct PieceList {
    pieces: Vec<(Piece, Square)>,  // active pieces with their squares
}
```

**Key operations**:
- **Move**: Update the square for the moving piece — O(n) to find, but n <= 16
- **Capture**: Remove captured piece by swapping with the last element and truncating — O(1)
- **Iteration**: Loop over only existing pieces, not empty squares

Piece lists complement mailbox or bitboards. With bitboards, you can derive piece locations via bitscan, making explicit piece lists somewhat redundant. But piece lists give you O(1) random access to a specific piece's location if indexed properly.

### Hybrid Approaches

Most serious chess programs use **redundant representations** updated in sync:

```
Bitboards  →  fast attack computation, piece iteration
Mailbox    →  fast "what's on this square?" lookup
Piece list →  fast piece-specific iteration (optional with bitboards)
```

The cost is maintaining consistency across all representations during every move. The benefit is each operation uses the most efficient data structure for its purpose.

---

## Game State

Beyond piece placement, a chess position requires additional state to be fully defined. This is exactly the information encoded in a FEN string.

### Side to Move

Which color plays next. In standard chess, white moves first and colors alternate.

### Castling Rights

Four independent boolean flags: white kingside (K), white queenside (Q), black kingside (k), black queenside (q). Efficiently stored as a 4-bit mask:

```rust
const WHITE_KINGSIDE: u8  = 0b0001;
const WHITE_QUEENSIDE: u8 = 0b0010;
const BLACK_KINGSIDE: u8  = 0b0100;
const BLACK_QUEENSIDE: u8 = 0b1000;
```

**When to revoke rights**:
- King moves → revoke both sides for that color
- Rook moves from its starting square → revoke that side
- Rook is captured on its starting square → revoke that side

A common technique: maintain a 64-element array where `castling_mask[sq]` holds the bits to AND with current rights after any move touching that square.

### En Passant Target

The square "behind" a pawn that just made a double push. If white plays e2-e4, the en passant target is e3. Only valid for one ply — cleared after the next move regardless of whether en passant was exercised.

### Halfmove Clock

Counts half-moves (ply) since the last pawn push or capture. When it reaches 100 (50 full moves), the game can be claimed as a draw under the fifty-move rule. Reset to 0 on any pawn move or capture.

### Fullmove Number

Starts at 1, increments after each Black move. Used for PGN output and display.

---

## Move Representation

### Move Encoding

A move can be encoded in a 16-bit integer:

```
Bits 0-5:   destination square (0-63)
Bits 6-11:  origin square (0-63)
Bits 12-13: promotion piece (0=knight, 1=bishop, 2=rook, 3=queen)
Bits 14-15: special flags (0=normal, 1=promotion, 2=en passant, 3=castling)
```

This compact representation allows move lists to be stored efficiently and passed around cheaply.

### Make / Unmake Move

Two strategies for applying and reversing moves:

**Incremental (Make/Unmake)**:
- `make_move()`: Apply the move, save irreversible state on a stack
- `unmake_move()`: Reverse the move using the saved state
- Advantage: memory-efficient, one board instance
- Disadvantage: complex unmake logic, must save en passant/castling/halfmove

**Copy-Make**:
- Copy the entire board, then apply the move to the copy
- Advantage: simple, no unmake needed (just discard the copy)
- Disadvantage: more memory, copying cost (mitigated if board is small)

Irreversible state (must be saved for unmake):
- Castling rights before the move
- En passant target before the move
- Halfmove clock before the move
- Captured piece (if any)

Reversible state (can be reconstructed):
- Piece positions (undo by moving back)
- Side to move (just flip)
- Fullmove number (decrement if needed)

---

## Attack Computation

### Non-Sliding Pieces

Knights, kings, and pawns have fixed attack patterns relative to their square. These can be precomputed into lookup tables:

```rust
// Precomputed at startup
const KNIGHT_ATTACKS: [u64; 64] = precompute_knight_attacks();
const KING_ATTACKS: [u64; 64] = precompute_king_attacks();
const PAWN_ATTACKS: [[u64; 64]; 2] = precompute_pawn_attacks(); // [Color][Square]
```

For a knight on e4 (square 28), `KNIGHT_ATTACKS[28]` gives a bitboard with bits set on all squares the knight can reach. No runtime computation needed.

### Sliding Pieces (Rooks, Bishops, Queens)

Sliding pieces are harder because their attacks depend on which squares are occupied (blocking). Several approaches:

**Classical/Iterative**: Walk along each ray direction, stopping at the first blocker. Simple but slow — requires loops.

**Rotated Bitboards**: Maintain separate bitboards rotated by 90, 45, and 315 degrees. Each rotation aligns a different set of attack rays with a rank, enabling lookup-table attacks. Complex to implement.

**Magic Bitboards**: The gold standard. Use a "magic number" multiplication to hash the relevant occupancy bits into an index for a precomputed attack table:

```
attacks = TABLE[square][(occupancy * MAGIC[square]) >> shift]
```

This gives O(1) sliding piece attacks at the cost of precomputed tables (~800KB). Extremely fast but complex to set up.

For ChessWAV, the classical iterative approach (which we already have in `path_clear()`) is sufficient since we don't need to search millions of positions per second.

---

## Position Identity

### Zobrist Hashing

A technique for incrementally maintaining a hash of the board position. Assign a random 64-bit number to every (piece, square) combination, plus side-to-move, castling rights, and en passant file:

```rust
hash ^= ZOBRIST[piece][color][square];  // toggle piece on/off
hash ^= ZOBRIST_SIDE;                   // toggle side to move
hash ^= ZOBRIST_CASTLING[rights];       // toggle castling
hash ^= ZOBRIST_EP[file];              // toggle en passant
```

Because XOR is its own inverse, making and unmaking a move both use the same XOR operations. This enables:
- **Threefold repetition detection**: Compare hashes in the move history
- **Transposition tables**: Cache evaluated positions by hash (for search engines)

---

## Relevance to ChessWAV

ChessWAV doesn't need a search engine. It processes games move-by-move from PGN input or REPL. But it still benefits from proper chess programming techniques:

| Technique | ChessWAV Benefit |
|-----------|-----------------|
| Bitboards | Eliminate O(64) scans in `find_origin()` |
| Game state | Track castling rights, en passant, side to move |
| Piece lists | Fast iteration over active pieces per color |
| Mailbox | Keep O(1) square queries for display and notation |
| Zobrist hashing | Future: detect repeated positions |
| Move encoding | Future: compact move history for undo/replay |

The hybrid mailbox + bitboards approach gives us the best of both worlds: fast parallel piece queries AND fast individual square lookups, with proper game state tracking that the current implementation lacks entirely.

---

## References

- [Chess Programming Wiki](https://chessprogramming.org/)
- [Board Representation](https://chessprogramming.org/Board_Representation)
- [Bitboards](https://chessprogramming.org/Bitboards)
- [Piece-Lists](https://chessprogramming.org/Piece-Lists)
- [0x88](https://chessprogramming.org/0x88)
- [Make Move](https://chessprogramming.org/Make_Move)
- [Zobrist Hashing](https://chessprogramming.org/Zobrist_Hashing)
- [Magic Bitboards](https://chessprogramming.org/Magic_Bitboards)
