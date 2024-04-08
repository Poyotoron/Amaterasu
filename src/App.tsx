import React, { useEffect, useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

import keyImage from "./assets/key.svg";
import keyImagePressed from "./assets/key_light.svg";
import scratchImage from "./assets/scratch.svg";

const scratchAreaLeft = 0
const scratchAreaTop = 0
const scratchAreaWidth = 400
const scratchAreaHeight = 400
const scratchImageWidth = 350
const scratchImageHeight = 350

const keyAreaLeft = 400
const keyAreaTop = 0
// const keyAreaWidth = 600
// const keyAreaHeight = 400
const keyImageWidth = 80
const keyImageHeight = 150

const statAreaLeft = 1000
const statAreaTop = 0
// const statAreaWidth = 400
// const statAreaHeight = 400

function App() {
  const [keyStates, setKeyStates] = useState([false, false, false, false, false, false, false]);
  const [keyCounts, setKeyCounts] = useState([0, 0, 0, 0, 0, 0, 0]);

  const [scratchState, setScratchState] = useState(0.0);
  const [scratchCount, setScratchCount] = useState(0);

  function createKeyStyle(keyNum: number) {
    // var index = keyNum / 2;
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
        <img src={scratchImage} style={{ position: "absolute", left: (scratchAreaWidth - scratchImageWidth) / 2, top: (scratchAreaHeight - scratchImageHeight) / 2, width: scratchImageWidth, height: scratchImageHeight, transform: `rotate(${-scratchState * 360}deg)` }} />
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

  class StatArea extends React.Component {
    render() {
      const keyCountViews = [];
      let keyCountSum = 0;
      for (var i = 0; i < 7; i++) {
        keyCountViews.push(<li>{keyCounts[i]}</li>);
        keyCountSum += keyCounts[i];
      }

      return (
        <div style={{ position: "absolute", left: statAreaLeft, top: statAreaTop, fontSize: 48 }}>
          <p>Key Count:</p>
          <p style={{ textAlign: "center" }}>{keyCountSum}</p>
          <p>Scratch Count:</p>
          <p style={{ textAlign: "center" }}>{scratchCount}</p>
        </div>
      );
    }
  }

  useEffect(() => {
    let listenButtonState: any;
    let listenButtonCounter: any;
    let listenScratchState: any;
    let listenScratchCount: any;

    async function f() {
      listenButtonState = await listen("buttonState", event => {
        console.log(event.payload);
        setKeyStates(event.payload as boolean[]);
      });

      listenButtonCounter = await listen("buttonCounter", event => {
        console.log(event.payload);
        setKeyCounts(event.payload as number[]);
      });

      listenScratchState = await listen("scratchState", event => {
        console.log(event.payload);
        setScratchState(event.payload as number);
      });

      listenScratchCount = await listen("scratchCount", event => {
        console.log(event.payload);
        setScratchCount(event.payload as number);
      });
    }
    f();

    return () => {
      if (listenButtonState) {
        listenButtonState();
      }

      if (listenButtonCounter) {
        listenButtonCounter();
      }

      if (listenScratchState) {
        listenScratchState();
      }

      if (listenScratchCount) {
        listenScratchCount();
      }
    }
  }, []);

  return (
    <>
      <ScratchArea /><KeyArea /><StatArea />
    </>
  );
}

export default App;
