Echo Server
=====

Echo Server

いくつかの言語で作成してみる基本的なechoサーバ
API仕様はdocフォルダ配下に置いてあります


## 基本的なAPI
- echo  
  送られたメッセージをそのまま表示します　　
- set  
  getによって取得できるメッセージを更新します  
- get
  setによって設定されたメッセージを取得します  


## 言語ごとのメモ
- rust  
  cargo
  http -> tokio, hyper, warp, reqwest
  gRPC -> tonic, prost, protobuf
- erlang  
  rebar3
  http -> cowboy
    (cowboyの参考)[https://cpplover.blogspot.com/2020/04/erlang-cowboy-rebar3hello-world-http.html]
  gRPC -> grpc, grpc_client


## 今後とか
OpenAPIを使用してAPIを定義しているけれど
gRPCで定義し直してみたい

