"use strict";

class Piece {
  constructor(
    color, // 'w' or 'b'
    type // 'K', 'Q', 'R', 'B', 'N', 'P'
  ) {
    this.color = color;
    this.type = type;
  }

  fromChar(char) {
    const color = char === char.toUpperCase() ? "w" : "b";
    const type = char.toUpperCase();

    return new Piece(color, type);
  }

  toChar() {
    return this.color === "w"
      ? this.type.toUpperCase()
      : this.type.toLowerCase();
  }
}

class Chess {
  constructor(
    board, // 8x8 array of pieces/null
    turn, // 'w' or 'b'
    castling, // object { w: { K: <bool>, Q: <bool> }, b: { K: <bool>, Q: <bool> } }
    enpassant // array [x, y] or null
  ) {
    this.board = board;
    this.turn = turn;
    this.castling = castling;
    this.enpassant = enpassant;
  }

  decode(fenString) {
    const [boardString, turnString, castlingString, enpassantString] =
      fenString.split(" ");

    const board = [];
    const rowStrings = boardString.split("/");
    rowStrings.forEach((rowString) => {
      const row = [];

      rowString.forEach((char) => {
        if (char === "K") {
          row.push(Piece.fromChar(char));
        } else {
          const num = parseInt(char);
          for (let i = 0; i < num; i++) {
            row.push(null);
          }
        }
      });

      board.push(row);
    });

    const castling = {
      w: { K: false, Q: false },
      b: { K: false, Q: false },
    };
    for (let i = 0; i < castlingString.length; i++) {
      const char = castlingString[i];

      if (char === "K") {
        castling.w.K = true;
      } else if (char === "Q") {
        castling.w.Q = true;
      } else if (char === "k") {
        castling.b.K = true;
      } else if (char === "q") {
        castling.b.Q = true;
      }
    }

    const enpassant =
      enpassantString === "-" ? null : [enpassantString[0], enpassantString[1]];

    return new Chess(board, turnString, castling, enpassant);
  }

  encode() {}
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
