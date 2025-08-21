import React, { useState } from 'react';
import { useGameState } from '../../lib/stores/useGameState';
import { useNarrative } from '../../lib/stores/useNarrative';
import { MiniMap } from './MiniMap';
import { QuestLog } from './QuestLog';
import { CharacterPanel } from './CharacterPanel';
import { InventoryPanel } from './InventoryPanel';
import { SanityMeter } from './SanityMeter';
import { StageIndicator } from './StageIndicator';
import { DialogueBox } from './DialogueBox';

interface GameHUDProps {
  isVisible: boolean;
}

export const GameHUD: React.FC<GameHUDProps> = ({ isVisible }) => {
  const { player, companions } = useGameState();
  const { currentStage, currentDialogue } = useNarrative();
  const [activePanel, setActivePanel] = useState<string | null>(null);

  if (!isVisible) return null;

  return (
    <div className="fixed inset-0 pointer-events-none z-10">
      {/* Top HUD Bar */}
      <div className="absolute top-4 left-4 right-4 flex justify-between items-start pointer-events-auto">
        {/* Left side - Player stats */}
        <div className="bg-black/80 backdrop-blur-sm rounded-lg p-3 border border-gray-600 min-w-[200px]">
          <div className="flex items-center gap-3 mb-2">
            <div className="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center text-white font-bold text-sm">
              {player.name[0]}
            </div>
            <div>
              <h3 className="text-white font-semibold text-sm">{player.name}</h3>
              <p className="text-gray-300 text-xs">Level {player.level}</p>
            </div>
          </div>
          
          {/* Health Bar */}
          <div className="mb-2">
            <div className="flex justify-between items-center mb-1">
              <span className="text-red-400 text-xs font-medium">HP</span>
              <span className="text-white text-xs">{player.health}/{player.maxHealth}</span>
            </div>
            <div className="w-full bg-gray-700 rounded-full h-2">
              <div 
                className="bg-red-500 h-2 rounded-full transition-all duration-300"
                style={{ width: `${(player.health / player.maxHealth) * 100}%` }}
              />
            </div>
          </div>

          {/* Experience Bar */}
          <div className="mb-2">
            <div className="flex justify-between items-center mb-1">
              <span className="text-blue-400 text-xs font-medium">XP</span>
              <span className="text-white text-xs">{player.experience}/{player.experienceToNext}</span>
            </div>
            <div className="w-full bg-gray-700 rounded-full h-2">
              <div 
                className="bg-blue-500 h-2 rounded-full transition-all duration-300"
                style={{ width: `${(player.experience / player.experienceToNext) * 100}%` }}
              />
            </div>
          </div>

          {/* Sanity Meter */}
          <SanityMeter />
        </div>

        {/* Right side - Stage indicator and companions */}
        <div className="flex flex-col gap-3">
          <StageIndicator stage={currentStage} />
          
          {/* Companion Status */}
          {companions.length > 0 && (
            <div className="bg-black/80 backdrop-blur-sm rounded-lg p-3 border border-gray-600">
              <h4 className="text-white text-sm font-semibold mb-2">Party</h4>
              <div className="flex flex-col gap-2">
                {companions.map((companion) => (
                  <div key={companion.id} className="flex items-center gap-2">
                    <div className="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center text-white text-xs">
                      {companion.name[0]}
                    </div>
                    <div className="flex-1">
                      <div className="text-white text-xs">{companion.name}</div>
                      <div className="w-16 bg-gray-700 rounded-full h-1">
                        <div 
                          className="bg-green-500 h-1 rounded-full"
                          style={{ width: `${(companion.morale / 100) * 100}%` }}
                        />
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Bottom HUD Bar */}
      <div className="absolute bottom-4 left-4 right-4 flex justify-between items-end pointer-events-auto">
        {/* Left side - Action buttons */}
        <div className="flex gap-2">
          <button
            onClick={() => setActivePanel(activePanel === 'quest' ? null : 'quest')}
            className="bg-black/80 backdrop-blur-sm rounded-lg p-3 border border-gray-600 hover:border-yellow-500 transition-colors"
            title="Quest Log"
          >
            <svg className="w-5 h-5 text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
              <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z"/>
              <path fillRule="evenodd" d="M4 5a2 2 0 012-2v1a1 1 0 001 1h6a1 1 0 001-1V3a2 2 0 012 2v6a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3z" clipRule="evenodd"/>
            </svg>
          </button>

          <button
            onClick={() => setActivePanel(activePanel === 'inventory' ? null : 'inventory')}
            className="bg-black/80 backdrop-blur-sm rounded-lg p-3 border border-gray-600 hover:border-blue-500 transition-colors"
            title="Inventory"
          >
            <svg className="w-5 h-5 text-blue-400" fill="currentColor" viewBox="0 0 20 20">
              <path d="M7 3a1 1 0 000 2h6a1 1 0 100-2H7zM4 7a1 1 0 011-1h10a1 1 0 110 2H5a1 1 0 01-1-1zM2 11a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2v-4z"/>
            </svg>
          </button>

          <button
            onClick={() => setActivePanel(activePanel === 'character' ? null : 'character')}
            className="bg-black/80 backdrop-blur-sm rounded-lg p-3 border border-gray-600 hover:border-green-500 transition-colors"
            title="Character"
          >
            <svg className="w-5 h-5 text-green-400" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clipRule="evenodd"/>
            </svg>
          </button>
        </div>

        {/* Right side - Minimap */}
        <div className="bg-black/80 backdrop-blur-sm rounded-lg p-2 border border-gray-600">
          <MiniMap />
        </div>
      </div>

      {/* Side Panels */}
      {activePanel === 'quest' && (
        <div className="absolute left-4 top-20 bottom-20 w-80 pointer-events-auto">
          <QuestLog onClose={() => setActivePanel(null)} />
        </div>
      )}

      {activePanel === 'inventory' && (
        <div className="absolute left-4 top-20 bottom-20 w-80 pointer-events-auto">
          <InventoryPanel onClose={() => setActivePanel(null)} />
        </div>
      )}

      {activePanel === 'character' && (
        <div className="absolute left-4 top-20 bottom-20 w-80 pointer-events-auto">
          <CharacterPanel onClose={() => setActivePanel(null)} />
        </div>
      )}

      {/* Dialogue Box */}
      {currentDialogue && (
        <div className="absolute bottom-20 left-1/2 transform -translate-x-1/2 w-full max-w-2xl pointer-events-auto">
          <DialogueBox dialogue={currentDialogue} />
        </div>
      )}
    </div>
  );
};