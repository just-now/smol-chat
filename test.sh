for i in {1..4000}; do
    echo "hello from $i" | APP=C ./target/debug/chat &
done
wait

