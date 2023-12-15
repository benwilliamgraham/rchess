"use strict";

class Piece {
  constructor(
    color, // 'w' or 'b'
    type // 'K', 'Q', 'R', 'B', 'N', 'P'
  ) {
    this.color = color;
    this.type = type;
  }
}

class Chess {
  constructor(
    board, // 8x8 array of pieces/null
    turn, // 'w' or 'b'
    castling, // object { w: { K: <bool>, Q: <bool> }, b: { K: <bool>, Q: <bool> } }
    enpassant, // array [x, y] or null
    halfmove, // number of halfmoves since last capture or pawn advance
    fullmove // number of fullmoves
  ) {
    this.board = board;
    this.turn = turn;
    this.castling = castling;
    this.enpassant = enpassant;
    this.halfmove = halfmove;
    this.fullmove = fullmove;
  }
}

async function main() {
  const response = await fetch(
    "./target/wasm32-unknown-unknown/release/rchess.wasm"
  );
  const { instance } = await WebAssembly.instantiateStreaming(response, {});

  const { test_fn } = instance.exports;

  console.log(test_fn());
}

main();
