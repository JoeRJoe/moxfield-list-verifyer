import { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { Clock, CheckCircle, XCircle, ChevronRight } from 'lucide-react';
import ResultCard from './ResultCard';

export default function History() {
  const [history, setHistory] = useState([]);
  const [selectedReport, setSelectedReport] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    fetch('/history')
      .then(res => res.json())
      .then(data => {
        setHistory(data);
        setIsLoading(false);
      })
      .catch(err => {
        console.error("Failed to fetch history:", err);
        setIsLoading(false);
      });
  }, []);

  if (selectedReport) {
    return (
      <div>
        <button
          onClick={() => setSelectedReport(null)}
          style={{ marginBottom: '1rem', display: 'flex', alignItems: 'center', gap: '0.5rem', background: 'transparent', border: 'none', color: 'var(--primary-color)', cursor: 'pointer' }}
        >
          ← Back to History
        </button>
        <ResultCard report={selectedReport} />
      </div>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      className="history-list"
      style={{ maxWidth: '800px', margin: '0 auto', textAlign: 'left' }}
    >
      <h2 style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <Clock /> Recent Analyses
      </h2>

      {isLoading ? (
        <p>Loading history...</p>
      ) : history.length === 0 ? (
        <p style={{ color: 'var(--secondary-color)' }}>No history found.</p>
      ) : (
        <div style={{ display: 'grid', gap: '1rem' }}>
          {history.map((report, index) => (
            <motion.div
              key={index}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.05 }}
              onClick={() => setSelectedReport(report)}
              className="card"
              style={{
                padding: '1rem',
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center',
                gap: '1rem',
                transition: 'transform 0.2s'
              }}
              whileHover={{ scale: 1.02 }}
            >
              {report.is_valid ? (
                <CheckCircle color="var(--success-color)" />
              ) : (
                <XCircle color="var(--error-color)" />
              )}
              <div style={{ flex: 1 }}>
                <h3 style={{ margin: 0, fontSize: '1.1rem' }}>{report.name}</h3>
                <p style={{ margin: 0, fontSize: '0.9rem', color: 'var(--secondary-color)' }}>by {report.author}</p>
              </div>
              <ChevronRight color="var(--secondary-color)" />
            </motion.div>
          ))}
        </div>
      )}
    </motion.div>
  );
}
