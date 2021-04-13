-module(kvs_server).

-behaviour(gen_server).

-export([start/0]).
-export([set/2, get/1, delete/1]).
-export([init/1, handle_call/3, handle_cast/2]).

start() ->
    gen_server:start_link({local, ?MODULE}, ?MODULE, [], []).

set(Key, Value) ->
    gen_server:call(?MODULE, {set, Key, Value}).

get(Key) ->
    gen_server:call(?MODULE, {get, Key}).

delete(Key) ->
    gen_server:call(?MODULE, {delete, Key}).

init(_Args) ->
    Storage = dict:new(),
    {ok, Storage}.

handle_cast(_Message, Storage) ->
    {noreply, Storage}.

handle_call({set, Key, Value}, _From, Storage) ->
    NewStorage = dict:store(Key, Value, Storage),
    {reply, ok, NewStorage};
handle_call({get, Key}, _From, Storage) ->
    case dict:is_key(Key, Storage) of
        true ->
            Value = dict:fetch(Key, Storage),
            {reply, Value, Storage};
        false ->
            {reply, none, Storage}
    end;
handle_call({delete, Key}, _From, Storage) ->
    case dict:is_key(Key, Storage) of
        true ->
            NewStorage = dict:erase(Key, Storage),
            {reply, ok, NewStorage};
        false ->
            {reply, ng, Storage}
    end.

