-module(tests).

-include_lib("eunit/include/eunit.hrl").

-export([test/0]).

test() ->
    server_test(),
    app_test().

server_test() ->
    kvs_server:start().

app_test() ->
    %% create, read and update
    kvs_server:set(key1, 10),
    ?assertEqual(10, kvs_server:get(key1)),
    kvs_server:set(key1, -1),
    ?assertEqual(-1, kvs_server:get(key1)),
    kvs_server:set(key2, <<"hello">>),
    ?assertEqual(<<"hello">>, kvs_server:get(key2)),
    ?assertEqual(ok, kvs_server:set(key3, assert)),
    ?assertEqual(assert, kvs_server:get(key3)),
    ?assertEqual(none, kvs_server:get(keyNone)),

    %% delete
    ?assertEqual(ng, kvs_server:delete(keyNone)),
    kvs_server:set(key4, atom),
    ?assertEqual(atom, kvs_server:get(key4)),
    ?assertEqual(ok, kvs_server:delete(key4)),
    ?assertEqual(none, kvs_server:get(key4)),

    ok.


