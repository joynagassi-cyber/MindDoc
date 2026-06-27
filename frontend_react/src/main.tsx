import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App';
import './index.css';

// SCY Forge — point d'entrée frontend (Vite + React 18)
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
