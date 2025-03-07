defmodule Player do
  require Logger

  def start(client_pid, username) do
    Logger.info("New player-(#{username}) spawned for client-(#{inspect(client_pid)})")
    stats = %{:hp => 10, :mp => 20}

    info = %{:x => 0.0, :y => 0.0}


    loop_player(stats, info)
  end

  def loop_player(stats, info) do
    receive do
      {:heal, value, pid} ->
        Logger.info("Player healed.")
        {value, _} = Integer.parse(value)
        {:ok, hp} = Map.fetch(stats, :hp)
        stats = Map.put(stats, :hp, hp + value)

        send(pid, {:client_send, "Healed for #{inspect(value)}."})
        loop_player(stats, info)

      {:stats, pid} ->
        send(pid, {:stats, stats, info})
        loop_player(stats, info)

      {:move, x, y, pid} ->
        send(pid, {:moved, x, y})
        {x_delta, _} = Integer.parse(x)
        {y_delta, _} = Integer.parse(y)

        {:ok, x} = Map.fetch(info, :x)
        info = Map.put(info, :x, x + x_delta)

        {:ok, y} = Map.fetch(info, :y)
        info = Map.put(info, :y, y + y_delta)

        loop_player(stats, info)

    after
      0 ->
        loop_player(stats, info)
    end
  end
end
