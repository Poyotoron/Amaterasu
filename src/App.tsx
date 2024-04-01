import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

const keyImage = "./src/assets/key.png"
const keyImagePressed = "./src/assets/key_light.png"
const scratchImage = "./src/assets/scratch.png"

const scratchAreaLeft = 0
const scratchAreaTop = 0
const scratchAreaWidth = 400
const scratchAreaHeight = 400
const scratchImageWidth = 300
const scratchImageHeight = 300

const keyAreaLeft = 400
const keyAreaTop = 0
const keyAreaWidth = 600
const keyAreaHeight = 400
const keyImageWidth = 80
const keyImageHeight = 150

function App() {
  // var keyStates = [false, false, false, false, false, false, false];
  const [keyStates, setKeyStates] = useState([false, false, false, false, false, false, false]);

  function createKeyStyle(keyNum: number) {
    var index = keyNum / 2;
    var isWhite = keyNum % 2 == 1;

    return {
      position: "absolute",
      left: 20 + keyNum * keyImageWidth,
      top: isWhite ? 50 : 225,
      width: keyImageWidth,
      height: keyImageHeight,
    };
  }

  class Scratch extends React.Component {
    render() {
      return (
        <img src={scratchImage} style={{ position: "absolute", left: 50, top: 50, width: scratchImageWidth, height: scratchImageHeight }} />
      );
    }
  }

  class ScratchArea extends React.Component {
    render() {
      return (
        <div style={{ position: "absolute", left: scratchAreaLeft, top: scratchAreaTop }}>
          <Scratch />
        </div>
      );
    }
  }

  class Key extends React.Component<{ src: string, keyNum: number }> {
    render() {
      return (
        <img src={this.props.src} style={createKeyStyle(this.props.keyNum) as React.CSSProperties} />
      )
    }
  }

  class Keys extends React.Component {
    render() {
      const keys = [];
      for (var i = 0; i < 7; i++) {
        if (keyStates[i]) {
          keys.push(<Key src={keyImagePressed} keyNum={i} />);
        } else {
          keys.push(<Key src={keyImage} keyNum={i} />);
        }
      }

      return (
        <span>
          {keys}
        </span>
      );
    }
  }

  class KeyArea extends React.Component {
    render() {
      return (
        <div style={{ position: "absolute", left: keyAreaLeft, top: keyAreaTop }}>
          <Keys />
        </div>
      );
    }
  }

  useEffect(() => {
    let unlisten: any;
    async function f() {
      unlisten = await listen("buttonState", event => {
        console.log(event.payload);
        setKeyStates(event.payload as boolean[]);
      });
    }
    f();

    return () => {
      if (unlisten) {
        unlisten();
      }
    }
  }, []);

  return (
    // <h1>Maaaaa</h1>
    <>
      <ScratchArea /><KeyArea />
    </>
  );
}

export default App;
