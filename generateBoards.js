const sg = require('./pkg/sudoku_generator');
const board_count = Number(process.argv[2]);

if (isNaN(board_count)) {
  console.error("\nUsage: node ./generateBoards.js [BOARD_COUNT]\n");
  process.exit(1);
}

for(let i = 0; i < board_count; i++) {
  sg.generate_and_fill_board(3, false)
}

