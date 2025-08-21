import React, { useState } from 'react';

interface DialogueOption {
  id: string;
  text: string;
  consequence?: string;
}

interface DialogueBoxProps {
  dialogue: {
    speaker: string;
    text: string;
    options?: DialogueOption[];
  };
}

export const DialogueBox: React.FC<DialogueBoxProps> = ({ dialogue }) => {
  const [selectedOption, setSelectedOption] = useState<string | null>(null);

  return (
    <div className="bg-black/90 backdrop-blur-sm rounded-lg border border-gray-600 p-4 max-w-2xl mx-auto">
      {/* Speaker */}
      <div className="flex items-center gap-3 mb-3">
        <div className="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center text-white font-bold text-sm">
          {dialogue.speaker[0]}
        </div>
        <h3 className="text-white font-semibold">{dialogue.speaker}</h3>
      </div>
      
      {/* Dialogue text */}
      <div className="text-gray-200 mb-4 leading-relaxed">
        {dialogue.text}
      </div>
      
      {/* Dialogue options */}
      {dialogue.options && dialogue.options.length > 0 && (
        <div className="space-y-2">
          {dialogue.options.map((option) => (
            <button
              key={option.id}
              onClick={() => setSelectedOption(option.id)}
              className={`w-full text-left p-3 rounded border transition-colors ${
                selectedOption === option.id
                  ? 'border-blue-500 bg-blue-900/30'
                  : 'border-gray-600 bg-gray-900/30 hover:bg-gray-800/50'
              }`}
            >
              <div className="text-white">{option.text}</div>
              {option.consequence && (
                <div className="text-xs text-gray-400 mt-1">
                  {option.consequence}
                </div>
              )}
            </button>
          ))}
        </div>
      )}
    </div>
  );
};