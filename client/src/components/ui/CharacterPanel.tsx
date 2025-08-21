import React from 'react';

interface CharacterPanelProps {
  onClose: () => void;
}

export const CharacterPanel: React.FC<CharacterPanelProps> = ({ onClose }) => {
  // Mock character data - would come from game state
  const character = {
    name: 'Adventurer',
    level: 1,
    health: 100,
    maxHealth: 100,
    experience: 0,
    experienceToNext: 100,
    stats: {
      strength: 10,
      dexterity: 12,
      intelligence: 14,
      constitution: 11,
      wisdom: 13,
      charisma: 15
    },
    skills: [
      { name: 'Persuasion', level: 2 },
      { name: 'Investigation', level: 1 },
      { name: 'Athletics', level: 1 }
    ]
  };

  return (
    <div className="bg-black/90 backdrop-blur-sm rounded-lg border border-gray-600 h-full flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-gray-600">
        <div className="flex items-center justify-between">
          <h3 className="text-white font-bold text-lg">Character</h3>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            ✕
          </button>
        </div>
      </div>
      
      {/* Content */}
      <div className="flex-1 p-4 overflow-y-auto">
        {/* Character info */}
        <div className="text-center mb-6">
          <div className="w-16 h-16 bg-blue-500 rounded-full flex items-center justify-center text-white font-bold text-xl mx-auto mb-2">
            {character.name[0]}
          </div>
          <h4 className="text-white font-bold text-lg">{character.name}</h4>
          <p className="text-gray-400">Level {character.level}</p>
        </div>
        
        {/* Stats */}
        <div className="mb-6">
          <h5 className="text-white font-semibold mb-3">Attributes</h5>
          <div className="grid grid-cols-2 gap-3">
            {Object.entries(character.stats).map(([stat, value]) => (
              <div key={stat} className="bg-gray-800 p-2 rounded">
                <div className="text-gray-400 text-xs capitalize">{stat}</div>
                <div className="text-white font-bold">{value}</div>
              </div>
            ))}
          </div>
        </div>
        
        {/* Skills */}
        <div>
          <h5 className="text-white font-semibold mb-3">Skills</h5>
          <div className="space-y-2">
            {character.skills.map((skill) => (
              <div key={skill.name} className="flex justify-between items-center bg-gray-800 p-2 rounded">
                <span className="text-white">{skill.name}</span>
                <span className="text-blue-400">Level {skill.level}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};