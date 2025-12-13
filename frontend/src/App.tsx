import { useState } from 'react';
import Hero from './components/Hero';
import InputForm from './components/InputForm';
import ResultCard from './components/ResultCard';
import History from './components/History';
import { AlertCircle, Home, Clock } from 'lucide-react';

function App() {
  const [activeTab, setActiveTab] = useState('home');
  const [report, setReport] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);
  const [progress, setProgress] = useState('');

  const handleValidate = async (id) => {
    setIsLoading(true);
    setError(null);
    setReport(null);
    setProgress('Connecting to validation server...');

    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';

    const wsUrl = `${protocol}//${window.location.hostname}:8000/ws/validate/${id}`;

    const ws = new WebSocket(wsUrl);

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === 'progress') {
          setProgress(data.status);
        } else if (data.is_valid !== undefined) {
          setReport(data);
          ws.close();
          setIsLoading(false);
        } else if (data.type === 'error') {
          setError(data.message);
          ws.close();
          setIsLoading(false);
        }
      } catch (e) {
        console.error("WS Parse error", e);
      }
    };

    ws.onerror = (e) => {
      console.error("WS Error", e);
      setError("Connection to validation server failed.");
      setIsLoading(false);
    };
  };

  return (
    <div className="App">
      <div style={{ display: 'flex', justifyContent: 'center', gap: '1rem', marginBottom: '2rem' }}>
        <button
          onClick={() => setActiveTab('home')}
          style={{
            background: activeTab === 'home' ? 'var(--primary-color)' : 'transparent',
            color: activeTab === 'home' ? 'white' : 'var(--secondary-color)',
            display: 'flex', alignItems: 'center', gap: '0.5rem'
          }}
        >
          <Home size={18} /> Home
        </button>
        <button
          onClick={() => setActiveTab('history')}
          style={{
            background: activeTab === 'history' ? 'var(--primary-color)' : 'transparent',
            color: activeTab === 'history' ? 'white' : 'var(--secondary-color)',
            display: 'flex', alignItems: 'center', gap: '0.5rem'
          }}
        >
          <Clock size={18} /> History
        </button>
      </div>

      {activeTab === 'home' ? (
        <>
          <Hero />
          <InputForm onSubmit={handleValidate} isLoading={isLoading} progress={progress} />

          {error && (
            <div style={{ color: 'var(--error-color)', marginBottom: '2rem', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '0.5rem' }}>
              <AlertCircle />
              {error}
            </div>
          )}

          {report && <ResultCard report={report} />}
        </>
      ) : (
        <History />
      )}
    </div>
  );
}

export default App;
