# Always a helperscript 

CPP=g++
CPPARGS="-Wall -Wextra -Werror -std=c++23 -g"

cd other/catstoy
$CPP $CPPARGS -o catstoy main.cpp
mv catstoy ../../catstoy

cd ../..

echo "Done building catstoy helperprogram"