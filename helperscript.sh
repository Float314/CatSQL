# Always a helperscript 

echo "==> CatsToy build helperscript"
CPP=g++
CPPARGS="-Wall -Wextra -std=c++23 -g"
echo "==> Arguments: C++: $CPPARGS "
echo "==> Using Compiler: $CPP"
cd other/catstoy
$CPP $CPPARGS -o catstoy main.cpp
mv catstoy ../../catstoy

cd ../..

echo "==>Done building catstoy helperprogram"
