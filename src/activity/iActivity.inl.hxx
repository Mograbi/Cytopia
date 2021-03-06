
template <typename StateType, typename... Args>
StateType & iActivity::createState(Args &&... args)
{
  StateType* state = new StateType(std::forward(args)..., GetService<UILoopMQ>());
  m_States.emplace_back(state);
  return *state;
}
