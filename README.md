# practice_rust

簡単なrustのコードを置いておく場所です




環境変数の設定
source sdkenv.sh

// Hello worldの作成
cargo new hello
cd hello
cargo builld
cp target/x86_64-wrs-vxworks/debug/hello.vxe ~/opt

// PythonTFTPサーバの起動
sudo python -m pyftpdlib -p 21 -u target -P vxTarget -i 127.0.0.1 -d $HOME

//
// QEMU版VxWorksの起動スクリプト

qemu-system-x86_64 -m 512M -kernel vxsdk/bsps/itl_generic_3_0_0_0/vxWorks \
-net nic -net user,hostfwd=tcp::1534-:1534,hostfwd=tcp::2345-:2345 \
-display none -serial stdio -monitor none \
-append "bootline:fs(0,0)host:vxWorks h=10.0.2.2 e=10.0.2.15 u=target pw=vxTarget o=gei0" 

// VxWorks起動後の手順
// コマンドモード
cmd
// 共有ディレクトリへ
cd opt

// VXEファイルを起動
 xxxx.vxe

変数　  hensu
プリミティブ型  primitive
フロー制御  flow
構造体  struct_impl
所有権  ownership_move
ジェネリクス  generics
クレート  crate_random
トレイト  trait_sample
マルチスレッド　multi_thread
