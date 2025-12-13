import { motion } from 'framer-motion';
import { ShieldCheck } from 'lucide-react';

export default function Hero() {
    return (
        <motion.div
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
            className="hero"
            style={{ marginBottom: '3rem' }}
        >
            <motion.div
                initial={{ scale: 0.8 }}
                animate={{ scale: 1 }}
                transition={{ delay: 0.2, type: 'spring' }}
                style={{ display: 'inline-block', padding: '1rem', background: 'rgba(59, 130, 246, 0.1)', borderRadius: '50%', marginBottom: '1rem' }}
            >
                <ShieldCheck size={64} color="var(--primary-color)" />
            </motion.div>
            <h1 style={{ background: 'linear-gradient(to right, #3b82f6, #8b5cf6)', WebkitBackgroundClip: 'text', WebkitTextFillColor: 'transparent', margin: '0.5rem 0' }}>
                Moxfield Verifyer
            </h1>
            <p style={{ color: 'var(--secondary-color)', fontSize: '1.2rem' }}>
                Valida la tua lista Moxfield secondo le regole EDH Nexus.
            </p>
        </motion.div>
    );
}
