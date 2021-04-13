-module(get_handler).

-behaviour(cowboy_handler).

-export([init/2]).

init(Req0, State) ->
    Msg = kvs_server:get(msg),
    Req = cowboy_req:reply(
        200,
        #{<<"content-type">> => <<"application/json; charset=utf-8">>},
        jsone:encode(#{<<"msg">> => Msg}),
        Req0
    ),
    {ok, Req, State}.

