defmodule Database do
  require Exqlite
  require Logger

  def start() do
    Logger.info("Starting up the database.")
    Process.register(self(), :database)
    Logger.info("Database alias registered.")

    {:ok, conn} = Exqlite.Sqlite3.open("../test.sqlite")

    a =
      Exqlite.Sqlite3.execute(
        conn,
        "create table players (id integer primary key, username text, password text)"
      )

    case a do
      :ok ->
        Logger.info("Created DB")

      {:error, _error} ->
        Logger.info("Already created db")
    end

    {:ok, statement} = Exqlite.Sqlite3.prepare(conn, "select count(id) from players")
    {:row, [player_count]} = Exqlite.Sqlite3.step(conn, statement)
    :ok = Exqlite.Sqlite3.release(conn, statement)

    Logger.info "DB Player count #{inspect(player_count)}"

    loop_db(conn, player_count)
  end

  def loop_db(conn, player_count) do
    receive do
      {:get, :player, player_name, pid} ->
        Logger.info("DB transaction")
        send(pid, {:player, false})

      {:new, :player, username, password, pid} ->
        {:ok, statement} =
          Exqlite.Sqlite3.prepare(
            conn,
            "insert into players (id, username, password) values (?1, ?2, ?3)"
          )

        :ok = Exqlite.Sqlite3.bind(statement, [player_count, username, password])

        :done = Exqlite.Sqlite3.step(conn, statement)
        :ok = Exqlite.Sqlite3.release(conn, statement)
        Logger.info("Player inserted.")

        send(pid, {:player, true})
        player_count = player_count + 1
        loop_db(conn, player_count)
    after
      1 ->
        loop_db(conn, player_count)
    end

    loop_db(conn, player_count)
  end
end
