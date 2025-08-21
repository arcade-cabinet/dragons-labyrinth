import React, { useState } from 'react';
import { useGameState } from '../../lib/stores/useGameState';
import { useNarrative } from '../../lib/stores/useNarrative';

interface QuestLogProps {
  onClose: () => void;
}

export const QuestLog: React.FC<QuestLogProps> = ({ onClose }) => {
  const { quests } = useGameState();
  const { currentStage } = useNarrative();
  const [selectedTab, setSelectedTab] = useState<'active' | 'completed' | 'failed'>('active');

  // Filter quests by status
  const activeQuests = quests.filter(q => q.status === 'active');
  const completedQuests = quests.filter(q => q.status === 'completed');
  const failedQuests = quests.filter(q => q.status === 'failed');

  // Get quest color based on type and urgency
  const getQuestColor = (quest: any) => {
    if (quest.type === 'main') return 'border-yellow-500 bg-yellow-900/20';
    if (quest.urgency === 'high') return 'border-red-500 bg-red-900/20';
    if (quest.urgency === 'medium') return 'border-orange-500 bg-orange-900/20';
    return 'border-gray-500 bg-gray-900/20';
  };

  // Get quest icon based on type
  const getQuestIcon = (quest: any) => {
    switch (quest.type) {
      case 'main':
        return '⭐';
      case 'companion':
        return '👥';
      case 'delivery':
        return '📦';
      case 'exploration':
        return '🗺️';
      case 'combat':
        return '⚔️';
      default:
        return '📋';
    }
  };

  const renderQuestList = (questList: any[]) => {
    if (questList.length === 0) {
      return (
        <div className="text-center text-gray-400 py-8">
          <p>No {selectedTab} quests</p>
        </div>
      );
    }

    return (
      <div className="space-y-3">
        {questList.map((quest) => (
          <div
            key={quest.id}
            className={`p-3 rounded-lg border ${getQuestColor(quest)} transition-all hover:bg-opacity-30`}
          >
            <div className="flex items-start gap-3">
              <span className="text-lg">{getQuestIcon(quest)}</span>
              <div className="flex-1">
                <div className="flex items-center gap-2 mb-1">
                  <h4 className="text-white font-semibold text-sm">{quest.title}</h4>
                  {quest.type === 'main' && (
                    <span className="px-2 py-0.5 bg-yellow-600 text-yellow-100 text-xs rounded-full">
                      Main
                    </span>
                  )}
                  {quest.urgency === 'high' && (
                    <span className="px-2 py-0.5 bg-red-600 text-red-100 text-xs rounded-full">
                      Urgent
                    </span>
                  )}
                </div>
                
                <p className="text-gray-300 text-xs mb-2 leading-relaxed">
                  {quest.description}
                </p>
                
                {/* Quest objectives */}
                {quest.objectives && quest.objectives.length > 0 && (
                  <div className="space-y-1 mb-2">
                    {quest.objectives.map((objective: any, index: number) => (
                      <div key={index} className="flex items-center gap-2 text-xs">
                        <span className={objective.completed ? 'text-green-400' : 'text-gray-400'}>
                          {objective.completed ? '✓' : '○'}
                        </span>
                        <span className={objective.completed ? 'text-green-300 line-through' : 'text-gray-300'}>
                          {objective.description}
                        </span>
                      </div>
                    ))}
                  </div>
                )}
                
                {/* Quest progress */}
                {quest.progress !== undefined && (
                  <div className="mb-2">
                    <div className="flex justify-between items-center mb-1">
                      <span className="text-xs text-gray-400">Progress</span>
                      <span className="text-xs text-white">{quest.progress}%</span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-1">
                      <div 
                        className="bg-blue-500 h-1 rounded-full transition-all duration-300"
                        style={{ width: `${quest.progress}%` }}
                      />
                    </div>
                  </div>
                )}
                
                {/* Quest rewards */}
                {quest.rewards && (
                  <div className="flex items-center gap-4 text-xs text-gray-400">
                    {quest.rewards.experience && (
                      <span>+{quest.rewards.experience} XP</span>
                    )}
                    {quest.rewards.gold && (
                      <span>+{quest.rewards.gold} Gold</span>
                    )}
                    {quest.rewards.items && quest.rewards.items.length > 0 && (
                      <span>+{quest.rewards.items.length} Items</span>
                    )}
                  </div>
                )}
                
                {/* Quest location hint */}
                {quest.location && quest.status === 'active' && (
                  <div className="mt-2 text-xs text-blue-400">
                    📍 {quest.location}
                  </div>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
    );
  };

  return (
    <div className="bg-black/90 backdrop-blur-sm rounded-lg border border-gray-600 h-full flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-gray-600">
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-white font-bold text-lg">Quest Log</h3>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            ✕
          </button>
        </div>
        
        {/* Tabs */}
        <div className="flex gap-1">
          {[
            { key: 'active', label: 'Active', count: activeQuests.length },
            { key: 'completed', label: 'Completed', count: completedQuests.length },
            { key: 'failed', label: 'Failed', count: failedQuests.length }
          ].map(tab => (
            <button
              key={tab.key}
              onClick={() => setSelectedTab(tab.key as any)}
              className={`px-3 py-1.5 text-sm rounded transition-colors ${
                selectedTab === tab.key
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
              }`}
            >
              {tab.label} ({tab.count})
            </button>
          ))}
        </div>
      </div>
      
      {/* Content */}
      <div className="flex-1 p-4 overflow-y-auto">
        {selectedTab === 'active' && renderQuestList(activeQuests)}
        {selectedTab === 'completed' && renderQuestList(completedQuests)}
        {selectedTab === 'failed' && renderQuestList(failedQuests)}
      </div>
      
      {/* Footer info */}
      <div className="p-3 border-t border-gray-600 bg-gray-900/50">
        <div className="text-xs text-gray-400 text-center">
          Stage {currentStage} • {activeQuests.length} active quest{activeQuests.length !== 1 ? 's' : ''}
        </div>
      </div>
    </div>
  );
};