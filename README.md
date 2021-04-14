Echo Server
=====

Echo Server

いくつかの言語で作成してみる基本的なechoサーバ
API仕様はdocフォルダ配下に置いてあります


## 基本的なAPI
- /  
  APIの定義をJSON形式でレスポンス  
- echo  
  送られたメッセージをそのまま表示します  
- set  
  getによって取得できるメッセージを更新します  
- get
  setによって設定されたメッセージを取得します  


### サンプル

- erlang版の起動
```sh
cd erlang
rebar3 eunit  # テスト
rebar3 shell  # 起動
```

- rust版の起動
```sh
cd rust
cargo test  # テスト
cargo run  # 起動
```

- APIの呼び出し
```sh
curl -i http://localhost:8080
curl -i -d msg=echo http://localhost:8080/echo
curl -i -d msg=echo http://localhost:8080/set
curl -i http://localhost:8080/get

# FIXME: Rust版はこっちじゃないと動かない
curl -i -H "Content-type: application/json" -d '{"msg":"echo"}' http://localhost:8080/echo
curl -i -H "Content-type: application/json" -d '{"msg":"echo"}' http://localhost:8080/set
```


## 言語ごとのメモ
- rust  
  cargo  
  http -> tokio, hyper, warp, reqwest, actix  
  gRPC -> tonic, prost, protobuf  
- erlang  
  rebar3  
  http -> cowboy, jsone  
    (cowboyの参考)[https://cpplover.blogspot.com/2020/04/erlang-cowboy-rebar3hello-world-http.html]  
  gRPC -> grpc, grpc_client  

