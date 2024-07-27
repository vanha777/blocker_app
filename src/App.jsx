import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";
import Integration from './components/integration/Integration'
import Sidebar from './components/sidebar/sidebar'
import MobileMenu from './components/mobileMenu/mobileMenu'
import Login from './components/login/login'
import Logs from './components/log/logs'
import ProfileSettings from './components/profileSetting/ProfileSetting'
import Loading from './components/loading/loading'
import DesktopMenu from './components/desktopMenu/desktopMenu'
import TimerClock from './components/timerClock/TimerClock'

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
  const [loading, setLoading] = useState(true);
  const [activeButton, setActiveButton] = useState(1);
  const [config, setConfig] = useState(null);
  const [save, setSave] = useState(false);
  const [user, setUser] = useState(false);
  const [tab, setTab] = useState(1);
  const changeTab = (index) => {
    console.log("changing tab")
    setTab(index)
  };
  const [count, setCount] = useState(0)
  const [message, setMessage] = useState()
  // const [config, setConfig] = useState()

  const callMessage = () => {
    invoke("greet", { name: "Roman" }).then((response) => setMessage(response))
  };

  const callCrash = () => {
    invoke("crash").then((response) => setMessage(response))
  };

  // const callReadConfig = () => {
  //   invoke("read_config").then((response) => {
  //     setConfig(response);
  //   }).catch((error) => {
  //     console.error('Error invoking read_config:', error);
  //     // setConfig('Failed to read config'); // or handle error as needed
  //   });
  // };

  return (
    <div data-theme="cupcake" className="flex flex-col min-h-screen"
    style={{
      // height: '38rem',
      // maxWidth: '640px',
      background: 'linear-gradient(to right, #E17AFE, #9BAAFF)'
    }}
    >


      {loading &&
        <div className="h-screen w-screen flex items-center justify-center">
          <Loading setUser={setUser} setLoading={setLoading} setConfig={setConfig} />
        </div>
      }

      {/* {!loading &&
        <MobileMenu activeButton={activeButton} setActiveButton={setActiveButton} />
      } */}

      {!loading &&
        <DesktopMenu activeButton={activeButton} setActiveButton={setActiveButton} />
      }

      {!user && !loading && config &&
        <div className="h-screen w-screen flex items-center justify-center">
          <Login user={user} setUser={setUser} config={config} setConfig={setConfig} />
        </div>
      }

      {config && user && activeButton === 1 &&
        <div className="w-screen flex items-center justify-center pb-10">
          <Integration config={config} setConfig={setConfig} />
        </div>
      }
      {config && user && activeButton === 2 &&
        <div className="mt-6 h-screen w-screen flex items-center justify-center">
          <Logs config={config} />
        </div>
      }
      {/* {config && user && activeButton === 3 &&
        <div className="pt-8 pb-8 h-screen w-screen flex items-center justify-center overflow-auto">
          <ProfileSettings config={config} setConfig={setConfig} setLoading={setLoading} />
        </div>
      } */}
      {config && user && activeButton === 3 &&
        <div className="pt-2 pb-2 h-screen w-screen flex items-center justify-center overflow-auto">
          <TimerClock config={config} setConfig={setConfig} setLoading={setLoading} />
        </div>
      }
      {/* {config && !save && tab == 1 &&
        <div className="toast toast-top toast-end space-y-2">
          <div className="alert alert-info p-4">
            <div className="flex flex-row items-center space-x-2">
              <svg onClick={() => changeTab(2)} xmlns="http://www.w3.org/2000/svg" className="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
              <span className="block text-sm md:text-base lg:text-lg">
                Please ensure that your settings are saved.
              </span>
              <button className="btn btn-sm btn-primary">Save</button>
            </div>
          </div>
        </div>
      } */}
      {/* {config && !save && tab == 2 &&
        <div onClick={() => changeTab(1)} className="toast toast-top toast-end space-y-2">
          <span className="loading loading-infinity loading-lg "></span>
        </div>
      } */}

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
    </div>
  )
}

export default App
