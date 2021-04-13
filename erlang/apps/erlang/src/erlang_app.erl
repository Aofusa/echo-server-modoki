%%%-------------------------------------------------------------------
%% @doc erlang public API
%% @end
%%%-------------------------------------------------------------------

-module(erlang_app).

-behaviour(application).

-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    Dispatch = cowboy_router:compile([
        {
            <<"localhost">>,
            [
                {<<"/">>, cowboy_static, {file, <<"../public/static/api.json">>}},
                {<<"/echo">>, echo_handler, []},
                {<<"/set">>, set_handler, []},
                {<<"/get">>, get_handler, []}
            ]
        }
    ]),
    kvs_server:start_link(),
    kvs_server:set(msg, <<"initialize msg">>),
    {ok, _} = cowboy:start_clear(
        echo_listener,
        [{port, 8080}],
        #{env => #{dispatch => Dispatch}}
    ),
    erlang_sup:start_link().

stop(_State) ->
    ok.

%% internal functions
