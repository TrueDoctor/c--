#Unit Test script
bin="inter-fuck"           # The application (from command arg)
diff="diff -iad"   # Diff command, or what ever

# An array, do not have to declare it, but is supposedly faster
declare -a file_base=("file1" "file2" "file3")

# Loop the array
for file in transpeter/tests/*.test; do
    # Padd file_base with suffixes
    base=${file%.test}
    file_in="$base.test"             # The in file
    file_out_val="$base.result"       # The out file to check against
    file_out_bf="$base.bf"       # The out file to check against
    file_out_tst="$base.out"   # The outfile from test application

    # Validate infile exists (do the same for out validate file)
    if [ ! -f "$file_in" ]; then
        printf "In file %s is missing\n" "$file_in"
        continue;
    fi
    if [ ! -f "$file_out_val" ]; then
        printf "Validation file %s is missing\n" "$file_out_val"
        continue;
    fi

    printf "Testing against %s\n" "$file_in"

    # Run application, redirect in file to app, and output to out file
    if python3 -o transpeter/main.py $file_in $file_out_bf; then
       python3 -o transpeter/main.py $file_in $file_out_bf
        ./$bin $file_out_bf > $file_out_tst
    else
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
    read -p "Enter a to abort, anything else to continue: " input_data
    # Iff input is "a" then abort
    [ "$input_data" == "a" ] && break

done

# Clean exit with status 0
exit 0

