#!/bin/bash
OUTFILE=./benchmark-results

rm $OUTFILE
touch $OUTFILE
make clean

make pkg
echo  ""
echo "Running Benchmarks for WASM ... "
echo  ""

for script in generateBoardsRsLoop.js generateBoards.js; do
  for board_size in 10 100 1000; do 
    echo "$script $board_size" >> $OUTFILE 
    for trial in 1 2 3 4 5; do 
      /usr/bin/time  --output .benchmark-tmp -v node $script $board_size; 
      cat .benchmark-tmp | grep "Maximum resident set size (kbytes):" >> $OUTFILE
      cat .benchmark-tmp | grep "Elapsed (wall clock) time (h:mm:ss or m:ss):" >> $OUTFILE
      echo ""  >> $OUTFILE
    done
  done
done
	

echo  ""
echo "Running Benchmarks for Native (OS rand) ... "
echo  ""
make build-native
for board_size in 10 100 1000; do
  echo "native $board_size";
  for trial in 1 2 3 4 5; do
      /usr/bin/time  --output .benchmark-tmp -v ./target/release/runner $board_size > /dev/null;
      cat .benchmark-tmp | grep "Maximum resident set size (kbytes):"  >> $OUTFILE
      cat .benchmark-tmp | grep "Elapsed (wall clock) time (h:mm:ss or m:ss):"  >> $OUTFILE
  done 
done

make clean build-native-thread-rng
echo  ""
echo "Running Benchmarks for Native (Thread rand) ... "
echo  ""
for board_size in 10 100 1000; do
  echo "native $board_size";
  for trial in 1 2 3 4 5; do
      /usr/bin/time  --output .benchmark-tmp -v ./target/release/runner $board_size > /dev/null;
      cat .benchmark-tmp | grep "Maximum resident set size (kbytes):" >> $OUTFILE
      cat .benchmark-tmp | grep "Elapsed (wall clock) time (h:mm:ss or m:ss):" >> $OUTFILE
  done
done >> $OUTFILE

rm -f .benchmark-tmp
