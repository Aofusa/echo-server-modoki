-module(set_handler).

-behaviour(cowboy_handler).

-export([init/2]).

init(Req0, State) ->
    Method = cowboy_req:method(Req0),
    HasBody = cowboy_req:has_body(Req0),
    Req = maybe_set(Method, HasBody, Req0),
    {ok, Req, State}.

maybe_set(<<"POST">>, true, Req0) ->
    {ok, PostVals, Req} = cowboy_req:read_urlencoded_body(Req0),
    Msg = proplists:get_value(<<"msg">>, PostVals),
    set(Msg, Req);
maybe_set(<<"POST">>, false, Req0) ->
    cowboy_req:reply(400, [], <<"Missing body.">>, Req0);
maybe_set(_, _, Req) ->
    %% Method not allowed.
    cowboy_req:reply(405, Req).

set(undefind, Req) ->
    cowboy_req:reply(400, [], <<"Missing parameter.">>, Req);
set(Msg, Req) ->
    kvs_server:set(msg, Msg),
    cowboy_req:reply(
        200,
        #{<<"content-type">> => <<"application/json; charset=utf-8">>},
        jsone:encode(#{<<"code">> => 0, <<"result">> => <<"success">>}),
        Req
    ).

