import Ability from '@ohos.app.ability.UIAbility'

export default class MainAbility extends Ability {
    onCreate(want, launchParam){
        // Ability is creating, initialize resources for this ability
        console.log('[Demo] MainAbility onCreate');
        globalThis.abilityWant = want;
    }

    onDestroy() {
        // Ability is destroying, release resources for this ability
        console.log('[Demo] MainAbility onDestroy');
    }

    onWindowStageCreate(windowStage) {
        // Main window is created, set main page for this ability
        console.log('[Demo] MainAbility onWindowStageCreate');
        globalThis.abilityContext = this.context;
        globalThis.cacheDir = this.context.cacheDir;
        windowStage.setUIContent(this.context, "MainAbility/pages/index/index", null);
    }

    onWindowStageDestroy() {
        //Main window is destroyed, release UI related resources
        console.log('[Demo] MainAbility onWindowStageDestroy');
    }

    onForeground() {
        // Ability has brought to foreground
        console.log('[Demo] MainAbility onForeground');
    }

    onBackground() {
        // Ability has back to background
        console.log('[Demo] MainAbility onBackground');
    }
};