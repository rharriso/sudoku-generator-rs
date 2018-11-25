const sg = require('./pkg/sudoku_generator');
const board_count = Number(process.argv[2]);

if (isNaN(board_count)) {
  console.error("\nUsage: node ./generateBoardsRsLoop.js [BOARD_COUNT]\n");
  process.exit(1);
}

sg.generate_and_fill_boards(board_count, 3, false);

