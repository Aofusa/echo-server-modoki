%%%-------------------------------------------------------------------
%% @doc erlang public API
%% @end
%%%-------------------------------------------------------------------

-module(erlang_app).

-behaviour(application).

-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    Dispatch = cowboy_router:compile([
        { <<"localhost">>, [{<<"/echo">>, echo_handler, []}]}
    ]),
    {ok, _} = cowboy:start_clear(
        echo_listener,
        [{port, 8080}],
        #{env => #{dispatch => Dispatch}}
    ),
    erlang_sup:start_link().

stop(_State) ->
    ok.

%% internal functions
