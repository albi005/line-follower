set -e

cargo build --release

scp ./target/armv5te-unknown-linux-musleabi/release/uwu ev3:~

echo -ne '\007'

