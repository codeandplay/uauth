docker build . -t uauth

id=$(docker create uauth)
docker cp $id:/app/target/x86_64-unknown-linux-gnu/release/uauth /tmp/uauth
docker rm -v $id

