import { useState } from 'react';
import { motion } from 'framer-motion';
import { Search, Loader2 } from 'lucide-react';

interface InputFormProps {
    onSubmit: (id: string) => void;
    isLoading: boolean;
    progress: string;
}

export default function InputForm({ onSubmit, isLoading, progress }: InputFormProps) {
    const [input, setInput] = useState('');

    const progressText = isLoading ? (progress || 'Initializing...') : '';

    const handleSubmit = (e) => {
        e.preventDefault();
        if (input.trim()) {
            let id = input.trim();
            const urlMatch = id.match(/moxfield\.com\/decks\/([a-zA-Z0-9_-]+)/);
            if (urlMatch) {
                id = urlMatch[1];
            }
            onSubmit(id);
        }
    };

    return (
        <motion.form
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            onSubmit={handleSubmit}
            style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', marginBottom: '3rem' }}
        >
            <div style={{ display: 'flex', flexWrap: 'wrap', alignItems: 'center', gap: '3rem', justifyContent: 'center', width: '100%' }}>
                <div style={{ position: 'relative', width: '100%', maxWidth: '400px' }}>
                    <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        placeholder="Enter Moxfield Deck URL or ID"
                        disabled={isLoading}
                        style={{ paddingLeft: '2.5rem' }}
                    />
                    <Search
                        size={18}
                        style={{ position: 'absolute', left: '0.8rem', top: '50%', transform: 'translateY(-50%)', color: 'var(--secondary-color)' }}
                    />
                </div>
                <div style={{ paddingLeft: '2rem' }}>
                    <button type="submit" disabled={isLoading || !input.trim()}>
                        {isLoading ? (
                            <Loader2 className="spin" size={20} />
                        ) : (
                            'Check'
                        )}
                    </button>
                </div>
            </div>
            {isLoading && (
                <motion.p
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    style={{ marginTop: '1rem', color: 'var(--secondary-color)', fontSize: '0.9rem' }}
                >
                    {progressText}
                </motion.p>
            )}
            <style>{`
        .spin {
          animation: spin 1s linear infinite;
        }
        @keyframes spin {
          from { transform: rotate(0deg); }
          to { transform: rotate(360deg); }
        }
      `}</style>
        </motion.form>
    );
}
