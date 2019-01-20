#Unit Test script
bin="inter-fuck"          # Brainfuck interpreter
diff='diff -ad -I '^#''   # Diff command,

# Loop the array
for file in transpeter/tests/*.test; do
    # Padd file_base with suffixes
    base=${file%.test}               # get the base filename
    file_in="$base.test"             # The in file
    file_in_val="$base.input"        # The input sequence file
    file_out_val="$base.result"      # The out file to check against
    file_out_bf="$base.bf"           # The out file to check against
    file_out_tst="$base.out"         # The outfile from test application

    # Validate infile exists (do the same for out validate file)
    if [ ! -f "$file_in" ]; then
        printf "In file %s is missing\n" "$file_in"
        continue;
    fi
    if [ ! -f "$file_out_val" ]; then
        printf "Validation file %s is missing\n" "$file_out_val"
        continue;
    fi

    printf "Testing against %s\n" "$file_in" ":"
    cat $file_out_val  # print expected output
    printf "\n"

    # Run application, redirect in file to app, and output to out file
    # First check wether cmm code compiles sucessfully
    if python3 transpeter/main.py -o $file_in $file_out_bf &> /dev/null; then
        python3 transpeter/main.py -o $file_in $file_out_bf
        # check wether an input sequence is given
        if [ -f "$file_in_val" ]; then
            cat $file_in_val | ./$bin $file_out_bf > $file_out_tst
        else
            ./$bin $file_out_bf > $file_out_tst
        fi
        # run test
    else
        # pipe compiler error into out file
        python3 transpeter/main.py $file_in 2> "$file_out_tst"
    fi


    # Execute diff
    $diff "$file_out_tst" "$file_out_val"


    # Check exit code from previous command (ie diff)
    # We need to add this to a variable else we can'it print it
    # as it will be changed by the if [
    # Iff not 0 then the files differ (at least with diff)
    e_code=$?
    if [ $e_code != 0 ]; then
            printf "TEST FAIL : %d\n" "$e_code"
    else
            printf "TEST OK!\n"
    fi

    # Pause by prompt
    if [ ! "$1" == "p" ]; then
        read -p "Enter a to abort, anything else to continue: " input_data
        # Iff input is "a" then abort
        [ "$input_data" == "a" ] && break
    fi

done

read -p "Enter k to keep temporary files, anything else to continue: " input_data
[ "$input_data" == "k" ] && exit 0

rm  transpeter/tests/*.bf
rm  transpeter/tests/*.out
# Clean exit with status 0
exit 0

