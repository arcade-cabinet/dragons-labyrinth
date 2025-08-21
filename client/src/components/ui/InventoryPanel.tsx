import React, { useState } from 'react';
import { useGameState } from '../../lib/stores/useGameState';

interface InventoryPanelProps {
  onClose: () => void;
}

export const InventoryPanel: React.FC<InventoryPanelProps> = ({ onClose }) => {
  const { player } = useGameState();
  const [selectedTab, setSelectedTab] = useState<'all' | 'weapons' | 'armor' | 'consumables' | 'misc'>('all');
  const [selectedItem, setSelectedItem] = useState<any>(null);

  // Mock inventory data - would come from game state
  const inventory = player.inventory || [
    {
      id: 'bread_1',
      name: 'Fresh Bread',
      type: 'consumable',
      rarity: 'common',
      quantity: 3,
      description: 'Warm bread from the village bakery. Restores health.',
      icon: '🍞',
      effects: { healing: 15 }
    },
    {
      id: 'rusty_sword',
      name: 'Rusty Sword',
      type: 'weapon',
      rarity: 'common',
      quantity: 1,
      description: 'An old sword with rust along the blade. Still sharp enough.',
      icon: '⚔️',
      stats: { attack: 8, durability: 45 }
    },
    {
      id: 'leather_armor',
      name: 'Leather Armor',
      type: 'armor',
      rarity: 'common',
      quantity: 1,
      description: 'Simple leather protection. Better than nothing.',
      icon: '🛡️',
      stats: { defense: 5, durability: 60 }
    },
    {
      id: 'ancient_key',
      name: 'Ancient Key',
      type: 'misc',
      rarity: 'rare',
      quantity: 1,
      description: 'A mysterious key with strange markings. What does it unlock?',
      icon: '🗝️'
    },
    {
      id: 'health_potion',
      name: 'Health Potion',
      type: 'consumable',
      rarity: 'uncommon',
      quantity: 2,
      description: 'A red potion that glows faintly. Restores significant health.',
      icon: '🧪',
      effects: { healing: 50 }
    }
  ];

  // Filter items by tab
  const getFilteredItems = () => {
    if (selectedTab === 'all') return inventory;
    return inventory.filter(item => item.type === selectedTab || 
      (selectedTab === 'consumables' && item.type === 'consumable'));
  };

  // Get rarity color
  const getRarityColor = (rarity: string) => {
    switch (rarity) {
      case 'common': return 'border-gray-500 bg-gray-900/20';
      case 'uncommon': return 'border-green-500 bg-green-900/20';
      case 'rare': return 'border-blue-500 bg-blue-900/20';
      case 'epic': return 'border-purple-500 bg-purple-900/20';
      case 'legendary': return 'border-yellow-500 bg-yellow-900/20';
      default: return 'border-gray-500 bg-gray-900/20';
    }
  };

  // Get item count by category
  const getCategoryCount = (category: string) => {
    if (category === 'all') return inventory.length;
    if (category === 'consumables') return inventory.filter(i => i.type === 'consumable').length;
    return inventory.filter(i => i.type === category).length;
  };

  const filteredItems = getFilteredItems();

  return (
    <div className="bg-black/90 backdrop-blur-sm rounded-lg border border-gray-600 h-full flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-gray-600">
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-white font-bold text-lg">Inventory</h3>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            ✕
          </button>
        </div>
        
        {/* Category tabs */}
        <div className="flex gap-1 flex-wrap">
          {[
            { key: 'all', label: 'All', icon: '📦' },
            { key: 'weapons', label: 'Weapons', icon: '⚔️' },
            { key: 'armor', label: 'Armor', icon: '🛡️' },
            { key: 'consumables', label: 'Items', icon: '🧪' },
            { key: 'misc', label: 'Misc', icon: '🎒' }
          ].map(tab => (
            <button
              key={tab.key}
              onClick={() => setSelectedTab(tab.key as any)}
              className={`px-2 py-1 text-xs rounded transition-colors flex items-center gap-1 ${
                selectedTab === tab.key
                  ? 'bg-blue-600 text-white'
                  : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
              }`}
            >
              <span>{tab.icon}</span>
              {tab.label} ({getCategoryCount(tab.key)})
            </button>
          ))}
        </div>
      </div>
      
      {/* Main content area */}
      <div className="flex-1 flex">
        {/* Items grid */}
        <div className="flex-1 p-4">
          {filteredItems.length === 0 ? (
            <div className="text-center text-gray-400 py-8">
              <p>No items in this category</p>
            </div>
          ) : (
            <div className="grid grid-cols-6 gap-2">
              {filteredItems.map((item) => (
                <div
                  key={item.id}
                  onClick={() => setSelectedItem(item)}
                  className={`relative p-2 rounded-lg border cursor-pointer transition-all hover:scale-105 ${
                    getRarityColor(item.rarity)
                  } ${selectedItem?.id === item.id ? 'ring-2 ring-blue-400' : ''}`}
                >
                  <div className="text-center">
                    <div className="text-2xl mb-1">{item.icon}</div>
                    <div className="text-xs text-white font-medium truncate">
                      {item.name}
                    </div>
                    {item.quantity > 1 && (
                      <div className="absolute -top-1 -right-1 bg-blue-600 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
                        {item.quantity}
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
        
        {/* Item details panel */}
        {selectedItem && (
          <div className="w-64 border-l border-gray-600 p-4 bg-gray-900/50">
            <div className="text-center mb-4">
              <div className="text-4xl mb-2">{selectedItem.icon}</div>
              <h4 className="text-white font-bold">{selectedItem.name}</h4>
              <div className={`text-sm capitalize font-medium ${
                selectedItem.rarity === 'common' ? 'text-gray-400' :
                selectedItem.rarity === 'uncommon' ? 'text-green-400' :
                selectedItem.rarity === 'rare' ? 'text-blue-400' :
                selectedItem.rarity === 'epic' ? 'text-purple-400' :
                'text-yellow-400'
              }`}>
                {selectedItem.rarity} {selectedItem.type}
              </div>
            </div>
            
            <div className="space-y-3">
              <p className="text-gray-300 text-sm">{selectedItem.description}</p>
              
              {/* Stats */}
              {selectedItem.stats && (
                <div>
                  <h5 className="text-white font-semibold text-sm mb-2">Stats</h5>
                  <div className="space-y-1">
                    {Object.entries(selectedItem.stats).map(([stat, value]) => (
                      <div key={stat} className="flex justify-between text-sm">
                        <span className="text-gray-400 capitalize">{stat}:</span>
                        <span className="text-white">{value as string}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
              
              {/* Effects */}
              {selectedItem.effects && (
                <div>
                  <h5 className="text-white font-semibold text-sm mb-2">Effects</h5>
                  <div className="space-y-1">
                    {Object.entries(selectedItem.effects).map(([effect, value]) => (
                      <div key={effect} className="flex justify-between text-sm">
                        <span className="text-gray-400 capitalize">{effect}:</span>
                        <span className="text-green-400">+{value as string}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
              
              {/* Quantity */}
              {selectedItem.quantity > 1 && (
                <div className="flex justify-between text-sm">
                  <span className="text-gray-400">Quantity:</span>
                  <span className="text-white">{selectedItem.quantity}</span>
                </div>
              )}
              
              {/* Action buttons */}
              <div className="space-y-2 pt-3">
                {selectedItem.type === 'consumable' && (
                  <button className="w-full bg-green-600 hover:bg-green-700 text-white py-2 px-3 rounded text-sm transition-colors">
                    Use Item
                  </button>
                )}
                {(selectedItem.type === 'weapon' || selectedItem.type === 'armor') && (
                  <button className="w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-3 rounded text-sm transition-colors">
                    Equip
                  </button>
                )}
                <button className="w-full bg-red-600 hover:bg-red-700 text-white py-2 px-3 rounded text-sm transition-colors">
                  Drop
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
      
      {/* Footer */}
      <div className="p-3 border-t border-gray-600 bg-gray-900/50">
        <div className="flex justify-between items-center text-xs text-gray-400">
          <span>{filteredItems.length} items</span>
          <span>Weight: {player.carryWeight || 0}/{player.maxCarryWeight || 100}</span>
        </div>
      </div>
    </div>
  );
};