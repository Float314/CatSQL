# Always a helperscript

EXECUTABLE=./target/debug/CatSQL
USAGE="./helperscript.sh [args]"

case $1 in 
    "run")
        $EXECUTABLE
        ;;
    "build")
        cargo build
        ;;
    "push")
        git push origin master
        ;;
    "license")
        echo "GNU LGPL v3"
        ;;
    *)
        echo unknown command: $1 
        echo usage: 
        echo $USAGE
        exit 1
        ;;
esac