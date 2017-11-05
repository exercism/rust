if [ ! -x bin/configlet ]; then
   echo "Improper configuration; configlet should exist in bin/ when this script is run"
   echo "Ping a Rust track maintainer to fix this"
   exit 1
fi

<<<<<<< HEAD
if [ ! -d "problem-specifications" ]; then
   git clone git@github.com:exercism/problem-specifications.git problem-specifications
fi

=======
>>>>>>> 28d4a17e7c424b47d9c6649c05cdd127c1a2ef59
newline=$'\n  '

missing_readmes=""
wrong_readmes=""
for exercise in $(git diff --name-only master..$(git rev-parse --abbrev-ref HEAD) | grep exercises/ | cut -d'/' -f2 -s | sort -fu); do
   echo "Checking readme for $exercise"
   readme_path="exercises/${exercise}/README.md"
   if [ ! -f $readme_path ]; then
      missing_readmes="$missing_readmes$newline$exercise"
   else
      existing_readme_checksum=$(md5sum $readme_path | cut -d' ' -f1)
      # generate the new README
<<<<<<< HEAD
      bin/configlet generate . --only "$exercise" --spec-path "problem-specifications"
=======
      bin/configlet generate . --only "$exercise"
>>>>>>> 28d4a17e7c424b47d9c6649c05cdd127c1a2ef59
      generated_readme_checksum=$(md5sum $readme_path | cut -d' ' -f1)

      if [ $existing_readme_checksum != $generated_readme_checksum ]; then
         wrong_readmes="$wrong_readmes$newline$exercise"
      fi
   fi
done

if [ -n "$missing_readmes" ]; then
  echo "Exercises missing README.md:$missing_readmes"
fi
if [ -n "$wrong_readmes" ]; then
  echo "Exercises with out-of-date README.md:$wrong_readmes"
fi
if [ -n "$missing_readmes" -o -n "$wrong_readmes" ]; then
   exit 1
fi
