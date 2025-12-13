import { motion } from 'framer-motion';
import { CheckCircle, XCircle, AlertTriangle } from 'lucide-react';

export default function ResultCard({ report }) {
    const isValid = report.is_valid;

    const container = {
        hidden: { opacity: 0 },
        show: {
            opacity: 1,
            transition: {
                staggerChildren: 0.1
            }
        }
    };

    const item = {
        hidden: { opacity: 0, y: 20 },
        show: { opacity: 1, y: 0 }
    };

    return (
        <motion.div
            variants={container}
            initial="hidden"
            animate="show"
            className="card"
            style={{ textAlign: 'left', maxWidth: '800px', margin: '0 auto' }}
        >
            <motion.div variants={item} style={{ display: 'flex', alignItems: 'center', gap: '1rem', marginBottom: '2rem', borderBottom: '1px solid var(--border-color)', paddingBottom: '1rem' }}>
                {isValid ? (
                    <CheckCircle size={48} color="var(--success-color)" />
                ) : (
                    <XCircle size={48} color="var(--error-color)" />
                )}
                <div>
                    <h2 style={{ margin: 0 }}>{report.name}</h2>
                    <p style={{ margin: 0, color: 'var(--secondary-color)' }}>by {report.author}</p>
                </div>
                <div style={{ marginLeft: 'auto', padding: '0.5rem 1rem', borderRadius: '2rem', background: isValid ? 'rgba(34, 197, 94, 0.1)' : 'rgba(239, 68, 68, 0.1)', color: isValid ? 'var(--success-color)' : 'var(--error-color)', fontWeight: 'bold' }}>
                    {isValid ? 'VALID' : 'INVALID'}
                </div>
            </motion.div>

            <div style={{ display: 'grid', gap: '1rem' }}>
                <Section title="Non-Land Tutors" items={report.non_land_tutors} icon={<AlertTriangle size={16} />} />
                <Section title="Mass Land Denial" items={report.mass_land_denial_cards} icon={<AlertTriangle size={16} />} />
                <Section title="Commander Tutors" items={report.commander_tutors.map(c => [c, 'Commander Tutor'])} icon={<AlertTriangle size={16} />} />
                <Section title="Two Card Combos" items={report.two_card_combos.map(c => [c[0].join(' + '), c[1]])} icon={<AlertTriangle size={16} />} />
                <Section title="Game Changers" items={report.gamechangers.map(c => [c, 'Game Changer'])} icon={<AlertTriangle size={16} />} />
                <Section title="Infinite Turns" items={report.infinite_turns_combos.map(c => [c.join(' + '), 'Infinite Turns'])} icon={<AlertTriangle size={16} />} />

                {report.deck_list && (
                    <div style={{ marginTop: '2rem', borderTop: '1px solid var(--border-color)', paddingTop: '1rem' }}>
                        <h3 style={{ fontSize: '1.2rem', marginBottom: '1rem' }}>Deck List</h3>
                        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))', gap: '0.5rem', maxHeight: '300px', overflowY: 'auto', paddingRight: '0.5rem' }}>
                            {report.deck_list.map((card, i) => (
                                <div key={i} style={{ background: 'rgba(0,0,0,0.2)', padding: '0.5rem', borderRadius: '4px', fontSize: '0.9rem', display: 'flex', justifyContent: 'space-between' }}>
                                    <span>{card.card}</span>
                                    <span style={{ color: 'var(--secondary-color)' }}>x{card.quantity}</span>
                                </div>
                            ))}
                        </div>
                    </div>
                )}
            </div>
        </motion.div>
    );
}

function Section({ title, items, icon }) {
    if (!items || items.length === 0) return null;

    return (
        <motion.div
            style={{ background: 'rgba(0,0,0,0.2)', padding: '1rem', borderRadius: '0.5rem' }}
        >
            <h3 style={{ margin: '0 0 0.5rem 0', fontSize: '1rem', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                {icon} {title}
            </h3>
            <ul style={{ margin: 0, paddingLeft: '1.5rem', color: 'var(--secondary-color)' }}>
                {items.map((item, i) => (
                    <li key={i}>
                        <span style={{ color: 'var(--text-color)' }}>{Array.isArray(item) ? item[0] : item}</span>
                        {Array.isArray(item) && item[1] && <span style={{ fontSize: '0.8em', opacity: 0.7 }}> - {item[1]}</span>}
                    </li>
                ))}
            </ul>
        </motion.div>
    );
}
