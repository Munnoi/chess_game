pub struct Piece {
    pub color: Color,
    pub kind: PieceType
};

// TODO: this is not supposed to be a 'struct' ds; fix it asap.
struct Color {
    white: String,
    black: String,
}

struct PieceType {
    king: String,
    queen: String,
    rook: String,
    bishop: String,
    knight: String,
    pawn: String,
}