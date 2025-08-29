# Dragon's Labyrinth Vendor Dependencies Makefile
# Manages external Godot addon dependencies using git subtree

# External addon repositories
GODOT_SQLITE_REPO = https://github.com/2shady4u/godot-sqlite.git
HEALTH_HITBOX_REPO = https://github.com/cluttered-code/godot-health-hitbox-hurtbox.git
LIMBOAI_REPO = https://github.com/limbonaut/limboai.git
QUEST_MANAGER_REPO = https://github.com/Rubonnek/quest-manager.git
QUEST_SYSTEM_REPO = https://github.com/shomykohai/quest-system.git
QUESTIFY_REPO = https://github.com/TheWalruzz/godot-questify.git
DIALOGIC_REPO = https://github.com/dialogic-godot/dialogic.git
BEEHAVE_REPO = https://github.com/bitbrain/beehave.git

.PHONY: all clean add-all remove-all status

# Reusable function to add/update external addons
define add-external-addon
	@if [ ! -d "./$(1)" ]; then \
		cd .. && git subtree add --prefix=vendor/$(1) $(2) $(3) --squash; \
	else \
		cd .. && git subtree pull --prefix=vendor/$(1) $(2) $(3) --squash; \
	fi
endef

define remove-external-addon
	@rm -rf ./$(1)
endef

# Main targets
all: add-all

add-all: add-sqlite add-health add-ai add-quests add-dialogue add-behavior

# Individual addon targets
add-sqlite:
	$(call add-external-addon,godot-sqlite,$(GODOT_SQLITE_REPO),master)

add-health:
	$(call add-external-addon,health-hitbox-hurtbox,$(HEALTH_HITBOX_REPO),main)

add-ai:
	$(call add-external-addon,limboai,$(LIMBOAI_REPO),master)

add-quests:
	$(call add-external-addon,quest-manager,$(QUEST_MANAGER_REPO),main)
	$(call add-external-addon,quest-system,$(QUEST_SYSTEM_REPO),main)
	$(call add-external-addon,questify,$(QUESTIFY_REPO),main)

add-dialogue:
	$(call add-external-addon,dialogic-external,$(DIALOGIC_REPO),main)

add-behavior:
	$(call add-external-addon,beehave,$(BEEHAVE_REPO),main)

# Remove targets
remove-sqlite:
	$(call remove-external-addon,godot-sqlite)

remove-health:
	$(call remove-external-addon,health-hitbox-hurtbox)

remove-ai:
	$(call remove-external-addon,limboai)

remove-quests:
	$(call remove-external-addon,quest-manager)
	$(call remove-external-addon,quest-system)
	$(call remove-external-addon,questify)

remove-dialogue:
	$(call remove-external-addon,dialogic-external)

remove-behavior:
	$(call remove-external-addon,beehave)

remove-all:
	@rm -rf ./godot-sqlite ./health-hitbox-hurtbox ./limboai 
	@rm -rf ./quest-manager ./quest-system ./questify
	@rm -rf ./dialogic-external ./beehave

status:
	@for addon in godot-sqlite health-hitbox-hurtbox limboai quest-manager quest-system questify dialogic-external beehave; do \
		if [ -d "./$$addon" ]; then \
			echo "✓ $$addon"; \
		else \
			echo "❌ $$addon"; \
		fi; \
	done

clean: remove-all
