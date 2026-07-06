// This is a temporary implementation for development of backend.

import { invoke } from '@tauri-apps/api/core';
import { useState, useRef, Dispatch, SetStateAction } from 'react';

async function setup() {
  try {
    await invoke('start_session', { filepath: '../test-binaries/basic.exe' });
    await invoke('analyze_binary');
  } catch (error) {
    console.error("[Backend Error]", error);
  }
}

function end() {
  invoke("close_application");
}

async function fillInformationState(setInfoContent: Dispatch<SetStateAction<{ "bin": {} }>>) {
  const data: string = await invoke("get_information");
  const parsed = JSON.parse(data);
  setInfoContent(parsed);
}

function App() {
  let [infoContent, setInfoContent] = useState({ "bin": {} });
  const infoModalRef = useRef<HTMLDialogElement>(null);

  return (
    <div>

      <button onClick={setup}>Press to setup!</button>
      <button onClick={end}>Press to stop!</button>

      <button className="btn" onClick={() => { fillInformationState(setInfoContent); infoModalRef.current?.showModal() }}>open modal</button>
      <dialog ref={infoModalRef} className="modal">
        <div className="modal-box">
          {infoContent ? Object.entries(infoContent).map(([key, value]) => (<p>{key}: {String(value)}</p>)) : "empty"}
          <div className="modal-action">
            <form method="dialog">
              <button className="btn">Close</button>
            </form>
          </div>
        </div>
      </dialog>
    </div>


  );
}

export default App;
