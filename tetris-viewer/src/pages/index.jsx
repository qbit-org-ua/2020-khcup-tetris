const COLORS = {
  I: "red",
  O: "blue",
  T: "darkorange",
  S: "green",
  Z: "darkcyan",
  L: "darkmagenta",
  J: "darkgrey",
};

const TETROMINO_BLOCKS = {
  I: [
    [
      [0, 0],
      [0, 1],
      [0, 2],
      [0, 3],
    ],
    [
      [0, 0],
      [1, 0],
      [2, 0],
      [3, 0],
    ],
    [
      [0, 0],
      [0, 1],
      [0, 2],
      [0, 3],
    ],
    [
      [0, 0],
      [1, 0],
      [2, 0],
      [3, 0],
    ],
  ],

  O: [
    [
      [0, 0],
      [1, 0],
      [0, 1],
      [1, 1],
    ],
    [
      [0, 0],
      [1, 0],
      [0, 1],
      [1, 1],
    ],
    [
      [0, 0],
      [1, 0],
      [0, 1],
      [1, 1],
    ],
    [
      [0, 0],
      [1, 0],
      [0, 1],
      [1, 1],
    ],
  ],

  T: [
    [
      [0, 0],
      [1, 0],
      [2, 0],
      [1, 1],
    ],
    [
      [1, 0],
      [1, 1],
      [1, 2],
      [0, 1],
    ],
    [
      [0, 1],
      [1, 1],
      [2, 1],
      [1, 0],
    ],
    [
      [0, 0],
      [0, 1],
      [0, 2],
      [1, 1],
    ],
  ],
  S: [
    [
      [0, 1],
      [1, 1],
      [1, 0],
      [2, 0],
    ],
    [
      [0, 0],
      [0, 1],
      [1, 1],
      [1, 2],
    ],
    [
      [0, 1],
      [1, 1],
      [1, 0],
      [2, 0],
    ],
    [
      [0, 0],
      [0, 1],
      [1, 1],
      [1, 2],
    ],
  ],
  Z: [
    [
      [0, 0],
      [1, 0],
      [1, 1],
      [2, 1],
    ],
    [
      [1, 0],
      [1, 1],
      [0, 1],
      [0, 2],
    ],
    [
      [0, 0],
      [1, 0],
      [1, 1],
      [2, 1],
    ],
    [
      [1, 0],
      [1, 1],
      [0, 1],
      [0, 2],
    ],
  ],
  J: [
    [
      [1, 0],
      [1, 1],
      [1, 2],
      [0, 2],
    ],
    [
      [0, 0],
      [0, 1],
      [1, 1],
      [2, 1],
    ],
    [
      [0, 0],
      [0, 1],
      [0, 2],
      [1, 0],
    ],
    [
      [0, 0],
      [1, 0],
      [2, 0],
      [2, 1],
    ],
  ],
  L: [
    [
      [0, 0],
      [0, 1],
      [0, 2],
      [1, 2],
    ],
    [
      [0, 0],
      [1, 0],
      [2, 0],
      [0, 1],
    ],
    [
      [0, 0],
      [1, 0],
      [1, 1],
      [1, 2],
    ],
    [
      [0, 1],
      [1, 1],
      [2, 1],
      [2, 0],
    ],
  ],
};

const Block = ({ color, lineIndex, columnIndex, style }) => {
  return (
    <>
      <div
        className="tetris-block"
        style={{
          backgroundColor: color,
          position: "absolute",
          top: lineIndex * 43,
          left: columnIndex * 43,
          ...style,
        }}
      ></div>
      <style jsx>{`
        .tetris-block {
          width: 43px;
          height: 43px;
        }
      `}</style>
    </>
  );
};

function emptyLine() {
  const line = [];
  for (let column = 0; column < 10; ++column) {
    line.push("");
  }
  return line;
}

class Tetris extends React.Component {
  state = {};

  init = (gameLog, callback) => {
    let board = [];
    for (let line = 0; line < 20; ++line) {
      board.push(emptyLine());
    }
    this.setState(
      {
        score: 0,
        persistentBoard: board.map((line) => line.slice()),
        board,
        logs: gameLog.split("\n"),
        actions: [],
        tetrominoKind: "",
        tetrominoLineIndex: -1,
        tetrominoColumnIndex: 0,
        tetrominoRotation: 0,
      },
      callback
    );
  };

  componentDidMount() {
    this.init(this.props.gameLog, this.playNext);
  }

  componentDidUpdate(prevProps) {
    if (this.props !== prevProps) {
      this.init(this.props.gameLog);
    }
  }

  fit = (board, lineIndex, blocks) => {
    for (const block of blocks) {
      if (
        block[1] > lineIndex ||
        board[lineIndex - block[1]][block[0]] !== ""
      ) {
        return false;
      }
    }
    return true;
  };

  playNext = () => {
    console.log("NEXT", this.state);
    if (this.state.gameOver) {
      return;
    }
    const newState = {};
    if (this.state.tetrominoLineIndex === -1) {
      const {
        logs: [tetromino, actions, ...logs],
      } = this.state;
      if (!tetromino) {
        return;
      }
      newState.logs = logs;
      const [tetrominoKind, tetrominoPosition] = tetromino.split(" ");
      newState.tetrominoKind = tetrominoKind;
      newState.tetrominoLineIndex = 19;
      newState.tetrominoColumnIndex = parseInt(tetrominoPosition) - 1;
      newState.tetrominoRotation = 0;
      newState.actions = actions.split(" ");
    } else if (this.state.actions.length > 0) {
      const {
        actions: [action, ...actions],
        tetrominoColumnIndex,
        tetrominoRotation,
      } = this.state;
      newState.actions = actions;

      switch (action) {
        case "shift_left":
          newState.tetrominoColumnIndex = tetrominoColumnIndex - 1;
          break;
        case "shift_right":
          newState.tetrominoColumnIndex = tetrominoColumnIndex + 1;
          break;
        case "rotate":
          newState.tetrominoRotation = tetrominoRotation + 1;
          break;
      }
    } else {
      let {
        tetrominoKind,
        tetrominoLineIndex,
        tetrominoColumnIndex,
        tetrominoRotation,
      } = this.state;
      --tetrominoLineIndex;
      let blocks = TETROMINO_BLOCKS[tetrominoKind][
        tetrominoRotation % 4
      ].map(([x, y]) => [tetrominoColumnIndex + x, y]);
      if (!this.fit(this.state.persistentBoard, tetrominoLineIndex, blocks)) {
        let { score } = this.state;
        const persistentBoard = this.state.board.map((line) => line.slice());
        for (let lineIndex = 0; lineIndex < 19; ++lineIndex) {
          if (persistentBoard[lineIndex].every((cell) => cell !== "")) {
            persistentBoard.copyWithin(lineIndex, lineIndex + 1);
            persistentBoard[19] = emptyLine();
            score += 1;
            --lineIndex;
          }
        }
        newState.persistentBoard = persistentBoard.slice();
        newState.score = score;
        newState.tetrominoLineIndex = -1;
        if (
          persistentBoard
            .slice(16, 19)
            .some((line) => line.some((cell) => cell !== ""))
        ) {
          newState.gameOver = true;
        }
        this.setState(newState);
        setTimeout(this.playNext, 500 / this.props.speed);
        return;
      }
      newState.tetrominoLineIndex = tetrominoLineIndex;
    }
    const {
      tetrominoKind,
      tetrominoLineIndex,
      tetrominoColumnIndex,
      tetrominoRotation,
    } = { ...this.state, ...newState };

    let blocks = TETROMINO_BLOCKS[tetrominoKind][tetrominoRotation % 4];

    blocks = blocks.map(([x, y]) => [tetrominoColumnIndex + x, y]);
    if (!blocks.every(([x, _]) => x <= 9)) {
      return;
    }

    const color = COLORS[tetrominoKind];

    const board = this.state.persistentBoard.map((line) => line.slice());
    for (const block of blocks) {
      board[tetrominoLineIndex - block[1]][block[0]] = color;
    }
    newState.board = board;

    this.setState(newState);
    setTimeout(this.playNext, 2000 / this.props.speed);
  };

  render() {
    if (this.state.score === undefined) {
      return null;
    }
    return (
      <>
        <div className="tetris-gameplay">
          {this.state.gameOver ? (
            <div className="tetris-gameover">
              <h2>The Game is Over!</h2>
              <p>(обновите страницу чтобы начать снова)</p>
            </div>
          ) : null}
          <div className="tetris-left-sidebar">
            <div className="tetris-level">9</div>
            <div className="tetris-score">{this.state.score}</div>
            <div className="tetris-input">
              <p>{`Current: ${this.state.tetrominoKind}`}</p>
              <p>{this.state.actions.join(" ")}</p>
            </div>
          </div>
          <div className="tetris-board">
            {this.state.board
              .slice()
              .reverse()
              .map((line, lineIndex) => {
                return line.map((cell, columnIndex) => {
                  return cell === "" ? null : (
                    <Block
                      key={`${lineIndex}-${columnIndex}`}
                      color={cell}
                      lineIndex={lineIndex}
                      columnIndex={columnIndex}
                    />
                  );
                });
              })}
          </div>
          <div className="tetris-right-sidebar"></div>
        </div>
        <style jsx global>{`
          @-moz-document url-prefix() {
            /* Disable scrollbar Firefox */
            html {
              scrollbar-width: none;
            }
          }
          body {
            margin: 0; /* remove default margin */
            scrollbar-width: none; /* Also needed to disable scrollbar Firefox */
            -ms-overflow-style: none; /* Disable scrollbar IE 10+ */
            overflow-y: scroll;
          }
          body::-webkit-scrollbar {
            width: 0px;
            background: transparent; /* Disable scrollbar Chrome/Safari/Webkit */
          }
          body {
            margin: 0;
            color: darkgrey;
            font-family: "DOS";
            font-size: 42px;
            font-weight: bold;
          }
          @font-face {
            font-family: "DOS";
            src: url("/static/perfect-dos-vga-437.otf") format("opentype");
          }
          .tetris-gameplay {
            background: url("/static/tetris-gameplay.png") no-repeat;
            width: 1920px;
            height: 1080px;
          }
          .tetris-gameover {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(255, 0, 0, 0.5);
            text-align: center;
            padding: 300px 0;
            z-index: 999;
          }
          .tetris-gameover h2 {
            font-size: 84px;
          }
          .tetris-left-sidebar,
          .tetris-board,
          .tetris-right-sidebar {
            float: left;
          }
          .tetris-left-sidebar {
            width: 700px;
            height: 300px;
            position: relative;
          }
          .tetris-level {
            position: absolute;
            left: 600px;
            top: 43px;
          }
          .tetris-score {
            position: absolute;
            left: 600px;
            top: 86px;
          }
          .tetris-input {
            position: absolute;
            left: 232px;
            top: 732px;
          }
          .tetris-board {
            /*background: rgba(202, 52, 52, 0.81);*/
            width: 430px;
            height: 870px;
            margin: 45px;
            position: relative;
          }
        `}</style>
      </>
    );
  }
}

export default class extends React.Component {
  state = {
    isStarted: false,
    gameLog: this.props.gameLog || "",
    gameLogUrl: this.props.gameLogUrl || "",
    speed: this.props.speed || 50,
  };

  static async getInitialProps({ query }) {
    return query;
  }

  startGame = (event) => {
    event.preventDefault();
    const { gameLogUrl } = this.state;
    if (gameLogUrl.length > 0) {
      fetch(gameLogUrl)
        .then((data) => data.text())
        .then((gameLog) => this.setState({ gameLog, isStarted: true }));
    } else {
      this.setState({ isStarted: true });
    }
  };

  updateStateValue = (event) => {
    this.setState({ [event.target.name]: event.target.value });
  };

  render() {
    const { gameLog, isStarted } = this.state;
    if (isStarted) {
      return <Tetris gameLog={gameLog} speed={this.state.speed} />;
    }
    return (
      <form onSubmit={this.startGame}>
        <p>
          <label>
            {"Введите лог игры: "}
            <textarea
              name="gameLog"
              value={gameLog}
              onChange={this.updateStateValue}
              cols="100"
              rows="20"
            />
          </label>
        </p>
        <p>
          <label>
            {"ИЛИ введите URL ссылку на лог игры: "}
            <input
              type="text"
              name="gameLogUrl"
              value={this.state.gameLogUrl}
              onChange={this.updateStateValue}
              style={{ width: 450 }}
            />
          </label>
        </p>
        <p>
          <label>
            {"Скорость игры (1 .. 500): "}
            <input
              type="number"
              name="speed"
              value={this.state.speed}
              min="1"
              max="500"
              onChange={this.updateStateValue}
            />
          </label>
        </p>
        <button type="submit">Старт!</button>
      </form>
    );
  }
}
