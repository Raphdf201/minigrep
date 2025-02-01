cargo build --release
mv ./target/release/searcher_txt searcher_txt
chmod +x searcher_txt
./searcher_txt test words.txt false