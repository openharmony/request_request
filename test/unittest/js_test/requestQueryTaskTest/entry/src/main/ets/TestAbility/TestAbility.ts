import Ability from '@ohos.app.ability.UIAbility'

export default class TestAbility extends Ability {
    onCreate(want, launchParam) {
        console.log('TestAbility onCreate');
    }

    onDestroy() {
        console.log('TestAbility onDestroy');
    }

    onWindowStageCreate(windowStage) {
        console.log('TestAbility onWindowStageCreate');
        windowStage.setUIContent(this.context, 'TestAbility/pages/index', null);

        globalThis.abilityContext = this.context;
    }

    onWindowStageDestroy() {
        console.log('TestAbility onWindowStageDestroy');
    }

    onForeground() {
        console.log('TestAbility onForeground');
    }

    onBackground() {
        console.log('TestAbility onBackground');
    }
};