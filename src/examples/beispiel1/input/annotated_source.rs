fn main(){
    let <tspan data-hash="1">x</tspan> = 5;
    let <tspan data-hash="2">y</tspan> = <tspan data-hash="1">x</tspan>;
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("x:{} y:{}", <tspan data-hash="1">x</tspan>, <tspan data-hash="2">y</tspan>);

    let <tspan data-hash="3">s1</tspan> = String::from("String1");

    let <tspan data-hash="4">s1_clone</tspan> = <tspan data-hash="3">s1</tspan>.<tspan class="fn" data-hash="0" hash="10">clone</tspan>(); 
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("s1 -> {}, s1_clone -> {}", <tspan data-hash="3">s1</tspan>, <tspan data-hash="4">s1_clone</tspan>);
    let <tspan data-hash="5">s2</tspan> = <tspan data-hash="3">s1</tspan>; 

    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("s2 -> {}", <tspan data-hash="5">s2</tspan>);
    <tspan class="fn" data-hash="0" hash="11">take_ownership</tspan>(<tspan data-hash="5">s2</tspan>); 

    let <tspan data-hash="6">s3</tspan> = <tspan class="fn" data-hash="0" hash="12">take_ownership_and_give_Back</tspan>(<tspan data-hash="4">s1_clone</tspan>);
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("s3 -> {}", <tspan data-hash="6">s3</tspan>);
} 

fn <tspan class="fn" data-hash="0" hash="11">take_ownership</tspan>(<tspan data-hash="7">string1</tspan>: String){ 
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("take_ownership string -> {}", <tspan data-hash="7">string1</tspan>);
} 

fn <tspan class="fn" data-hash="0" hash="12">take_ownership_and_give_Back</tspan>(<tspan data-hash="8">string2</tspan>: String) -> String {
    <tspan class="fn" data-hash="0" hash="9">println!</tspan>("take_ownership_and_give_back string -> {}", <tspan data-hash="8">string2</tspan>);
    <tspan data-hash="8">string2</tspan> 
}