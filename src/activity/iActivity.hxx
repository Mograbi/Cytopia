#ifndef I_ACTIVITY_HXX
#define I_ACTIVITY_HXX

#include <memory>
#include <betterEnums.hxx>

#include "../layout/iLayout.hxx"
#include "../view/iViewElement.hxx"
#include "../GameService.hxx"
#include "../model/iModel.hxx"
#include "../activity/ActivityType.hxx"

class iActivity : public GameService, public virtual iView
{
  class Window & m_Window;
  std::vector<iModelPtr> m_States;
  friend class Window;

public:
 
  iActivity(GameService::ServiceTuple &, class Window &);
  virtual ~iActivity() = 0;

protected:

  /**
   * @brief Switches to another iActivity
   * @param ActivityType the type of iActivity to switch to
   */
  void activitySwitch(ActivityType);

  /**
   * @brief Creates a new State
   * @tparam StateType the type of the state
   * @tparam Args arguments to construct the state
   * @returns a referenece to the created state
   */  
  template <typename StateType, typename... Args>
  StateType & createState(Args &&... args);

  /**
   * @returns the Window this Activity is assigned to 
   */
  Window & getWindow() noexcept;
};

using iActivityPtr = std::unique_ptr<iActivity>;

#include "iActivity.inl.hxx"

#endif // I_ACTIVITY_HXX
