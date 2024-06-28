array=("wasmer" "wasmtime" "wasmedge" "wasm3")

build() {
  echo "build"
  cd /home/stefano/projetcs/progetto_tesi/rust_benchmark
  for element in "${array[@]}"
  do
      echo "$element"
      cd $element
      cargo build --release
      cd ..
  done
}

run() {
  echo "run"
  cd /home/stefano/projetcs/progetto_tesi/rust_benchmark
  for element in "${array[@]}"
  do
      echo "$element"
      cd $element
      cargo run --release
      cd ..
  done
}

clean() {
  echo "clean"
  cd /home/stefano/projetcs/progetto_tesi/rust_benchmark
  for element in "${array[@]}"
  do
      echo "$element"
      cd $element
      cargo clean
      cd ..
  done
}


if [ $1 = "build" ]; then
  build
elif [ $1 = "run" ]; then
  run
elif [ $1 = "clean" ]; then
  clean
fi