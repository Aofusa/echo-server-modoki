-module(echo_handler).

-behaviour(cowboy_handler).

-export([init/2]).

init(Req0, State) ->
    Req = cowboy_req:reply(
        200,
        #{<<"content-type">> => <<"application/json">>},
        jsone:encode(#{<<"message">> => <<"hello,world">>}),
        Req0
    ),
    {ok, Req, State}.

