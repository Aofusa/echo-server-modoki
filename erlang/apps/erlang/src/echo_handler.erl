-module(echo_handler).

-behaviour(cowboy_handler).

-export([init/2]).

init(Req0, State) ->
    Method = cowboy_req:method(Req0),
    HasBody = cowboy_req:has_body(Req0),
    Req = maybe_echo(Method, HasBody, Req0),
    {ok, Req, State}.

maybe_echo(<<"POST">>, true, Req0) ->
    {ok, PostVals, Req} = cowboy_req:read_urlencoded_body(Req0),
    Echo = proplists:get_value(<<"msg">>, PostVals),
    echo(Echo, Req);
maybe_echo(<<"POST">>, false, Req) ->
    cowboy_req:reply(400, [], <<"Missing body.">>, Req);
maybe_echo(_, _, Req) ->
    %% Method not allowed.
    cowboy_req:reply(405, Req).

echo(undefind, Req) ->
    cowboy_req:reply(400, [], <<"Missing parameter.">>, Req);
echo(Echo, Req) ->
    cowboy_req:reply(
        200,
        #{<<"content-type">> => <<"application/json; charset=utf-8">>},
        jsone:encode(#{<<"msg">> => Echo}),
        Req
    ).

