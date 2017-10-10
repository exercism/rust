current_dir=$(pwd)

cd exercises

for exercise_name in *
do

#Get filename
file_name="$exercise_name/tests/*.rs"

#Commented tests
comment_test_num=$(cat $file_name | grep "// \#\[test\]" | wc -l)

#Number of tests
test_num=$(cat $file_name | grep "\#\[test\]" | wc -l)
test_num=$(($test_num - $comment_test_num))

#Commented ignores
comment_ignore_num=$(cat $file_name | grep "// \#\[ignore\]" | wc -l)

#Number of ignores
ignore_num=$(cat $file_name | grep "\#\[ignore\]" | wc -l)
ignore_num=$(($ignore_num - $comment_ignore_num))

#Number of tests minus 1 should be the number of ignores
compare=$(($test_num - 1))

#Print logic
if [ "$compare" -eq "$ignore_num" ]
then
    #Valid
    echo -n
else
    echo "$file_name : "
    echo "Tests: $test_num"
    echo "Ignores: $ignore_num"
fi
done

cd $current_dir
