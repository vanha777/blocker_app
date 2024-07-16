import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";
import Integration from './components/integration/Integration'
import Sidebar from './components/sidebar/sidebar'
import MobileMenu from './components/mobileMenu/mobileMenu'

function App() {

  // useEffect(() => {
  //   auto_start();
  // }, []);

  const auto_start = async () => {
    // set auto-start  -> only required first-time -> will be vary on OS ...
    await enable();
    console.log(`registered for autostart? ${await isEnabled()}`);
    // disable();
    //end.
  };
  const [activeButton, setActiveButton] = useState(1);
  const [user, setUser] = useState(null);
  const [tab, setTab] = useState(1);
  const changeTab = (index) => {
    console.log("changing tab")
    setTab(index)
};
  const [count, setCount] = useState(0)
  const [message, setMessage] = useState()
  const [config, setConfig] = useState()

  const callMessage = () => {
    invoke("greet", { name: "Roman" }).then((response) => setMessage(response))
  };

  const callCrash = () => {
    invoke("crash").then((response) => setMessage(response))
  };

  const callReadConfig = () => {
    invoke("read_config").then((response) => {
      setConfig(response);
    }).catch((error) => {
      console.error('Error invoking read_config:', error);
      // setConfig('Failed to read config'); // or handle error as needed
    });
  };

  return (
    <>
            <div>
            <MobileMenu activeButton={activeButton} setActiveButton={setActiveButton} />
            {activeButton === 1 &&
                <div className="flex items-center justify-center pb-10">
                     <Integration />
                </div>
            }
            {activeButton === 2 &&
                <div className="h-screen w-screen flex items-center justify-center">
                    {/* <WalletCard /> */}
                </div>
            }
            {activeButton === 3 &&
                <div className="h-screen w-screen flex items-center justify-center">
                    {/* <Connection /> */}
                </div>
            }
            {/* {!user && (
                <div className="toast toast-top toast-end">
                    <div role="alert" className="alert alert-info">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="stroke-info shrink-0 w-6 h-6"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                       <span>Claim your account so that you don't lose your stuff!</span>
                </div>
                        <div className="">
                            <button className="btn btn-sm btn-primary">Sign up!</button>
                            <button className="btn btn-sm">
                                <IconClose />
                            </button>
                        </div>
                    </div>
                </div>
            )} */}
            {!user && tab == 1 &&
                <div className="toast toast-top toast-end space-y-2">
                    <div className="alert alert-info p-4">
                        <div className="flex flex-row items-center space-x-2">
                            <svg onClick={() => changeTab(2)} xmlns="http://www.w3.org/2000/svg" className="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                            <span className="block text-sm md:text-base lg:text-lg">
                                Claim your account now !
                            </span>
                            <button className="btn btn-sm btn-primary">Sign up!</button>
                        </div>
                    </div>
                </div>
            }
            {!user && tab == 2 &&
                <div onClick={() => changeTab(1)} className="toast toast-top toast-end space-y-2">
                    <span className="loading loading-infinity loading-lg "></span>
                </div>
            }


            {/* <div className="flex flex-col lg:flex-row gap-20 items-center justify-center align-middle">
                <div className="animate-in flex-1 flex flex-col gap-20 opacity-0 max-w-4xl px-3">
                    <main className="flex-1 flex flex-col lg:flex-row gap-6">
                        <div className="flex flex-col lg:flex-row lg:space-x-4 space-y-4"> */}
            {/* <ProfileCard /> */}
            {/* <WalletCard /> */}
            {/* </div>
                    </main>
                </div>
            </div> */}

        </div>
     

      {/* <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>This Is Banana
      </h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <button onClick={() => callMessage()}>
          Rust function : {message}
        </button>
        <button onClick={() => callCrash()}>
          Crash function
        </button>
        <button onClick={() => auto_start()}>
          Auto-start
        </button>
        <button onClick={() => callReadConfig()}>
          Config File : {JSON.stringify(config)}
          Read Config
        </button>
        <p>
          Edit <code>src/App.jsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p> */}
    </>
  )
}

export default App
