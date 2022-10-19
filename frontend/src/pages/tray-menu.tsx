import type { NextPage } from 'next';
import { showMainWindow } from '../services/tauri';

const TrayMenu: NextPage = () => {
  const onAppOpenClick = async () => {
    await showMainWindow();
  };

  return (
    <div className='bg-gray-900 flex flex-col grow justify-between p-2'>
      <div>
        <h2>Tray Menu</h2>
      </div>

      {/* Menu buttons */}
      <div>
        <button onClick={onAppOpenClick}>Open App</button>
        <button>Quit</button>
      </div>
    </div>
  );
};

export default TrayMenu;
